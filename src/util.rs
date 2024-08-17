use std::{fmt::Display, ops::Deref, sync::Arc, time::{Duration, SystemTime}};

use backon::BackoffBuilder;
use thiserror::Error;
use tokio::{select, sync::{broadcast, mpsc, oneshot, Mutex}, task::JoinHandle};

use crate::error::RelayError;
use futures::FutureExt;

pub trait Resource: Send + Sync + Sized {
    // resolve when resource is done
    fn generate(self: &Arc<Self>) -> impl std::future::Future<Output = Result<JoinHandle<()>, RelayError>> + Send;

    fn generate_unwind_safe(self: &Arc<Self>) -> impl std::future::Future<Output = Result<JoinHandle<()>, RelayError>> + Send {
        async {
            std::panic::AssertUnwindSafe(self.generate())
                .catch_unwind().await
                .map_err(|e| {
                    println!("paniced with {:?}", e.downcast_ref::<&str>());
                    RelayError::ResourcePanic(e.downcast_ref::<&str>().unwrap_or(&"failed to str!").to_string())
                })
                .and_then(|a| a)
        }
    }
}

const MAX_RESOURCE_REGEN: Duration = Duration::from_secs(15);
const MAX_RESOURCE_WAIT: Duration = Duration::from_secs(30);

pub struct ResourceManager<T: Resource> {
    pub resource: Arc<T>,
    refreshed_at: Mutex<SystemTime>,
    request_retries: mpsc::Sender<oneshot::Sender<Result<(), Arc<RelayError>>>>,
    retry_signal: mpsc::Sender<()>,
    retry_now_signal: mpsc::Sender<()>,
    death_signal: Option<mpsc::Sender<()>>,
    pub generated_signal: broadcast::Sender<()>,
    pub resource_state: Mutex<ResourceState>,
}

impl<T: Resource> Deref for ResourceManager<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

impl<T: Resource> Drop for ResourceManager<T> {
    fn drop(&mut self) {
        let my_ref = self.death_signal.take().expect("Death empty; already dropped?");
        tokio::spawn(async move {
            my_ref.send(()).await.unwrap()
        });
    }
}

#[derive(Clone, Debug, Error)]
pub struct ResourceFailure {
    pub retry_wait: Option<u64>,
    pub error: Arc<RelayError>,
}

impl Display for ResourceFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to generate resource {}; {}", self.error, 
            if let Some(retry_in) = self.retry_wait { format!("retrying in {}s", retry_in) } else { "not retrying".to_string() })
    }
}


#[derive(Clone)]
pub enum ResourceState {
    Generated,
    Generating,
    Failed (ResourceFailure)
}

impl<T: Resource + 'static> ResourceManager<T> {
    pub fn new<B: BackoffBuilder + 'static>(resource: Arc<T>, backoff: B, running_resource: Option<JoinHandle<()>>) -> Arc<ResourceManager<T>> {
        let (retry_send, mut retry_recv) = mpsc::channel::<oneshot::Sender<Result<(), Arc<RelayError>>>>(99999);
        let (sig_send, mut sig_recv) = mpsc::channel(99999);
        let (retry_now_send, mut retry_now_recv) = mpsc::channel(99999);
        let (death_send, mut death_recv) = mpsc::channel(99999);
        let (generated_send, _) = broadcast::channel(99);

        let manager = Arc::new(ResourceManager {
            resource,
            refreshed_at: Mutex::new(SystemTime::UNIX_EPOCH),
            request_retries: retry_send,
            retry_signal: sig_send,
            retry_now_signal: retry_now_send,
            death_signal: Some(death_send),
            generated_signal: generated_send.clone(),
            resource_state: Mutex::new(if running_resource.is_some() { ResourceState::Generated } else { ResourceState::Generating }),
        });

        let mut current_resource = running_resource.unwrap_or_else(|| tokio::spawn(async {}));

        let loop_manager = manager.clone();
        tokio::spawn(async move {
            let mut resolve_items = move |result: Result<(), Arc<RelayError>>, sig_recv: &mut mpsc::Receiver<()>, sig_recv_now: &mut mpsc::Receiver<()>| {
                while let Ok(_) = sig_recv.try_recv() { }
                while let Ok(_) = sig_recv_now.try_recv() { }
                while let Ok(item) = retry_recv.try_recv() {
                    let _ = item.send(result.clone());
                }
            };

            'stop: loop {
                select! {
                    _ = &mut current_resource => {},
                    _ = sig_recv.recv() => {},
                    _ = retry_now_recv.recv() => {},
                    _ = death_recv.recv() => {
                        break // no retries
                    },
                }
                current_resource.abort();
                let mut backoff = backoff.build();
                *loop_manager.resource_state.lock().await = ResourceState::Generating;
                let mut result = loop_manager.resource.generate_unwind_safe().await;
                while let Err(e) = result {

                    println!("resource failed with {e}");
                    
                    let shared_err = Arc::new(e);
                    resolve_items(Err(shared_err.clone()), &mut sig_recv, &mut retry_now_recv);
                    let retry_in = backoff.next().unwrap();

                    let is_final = matches!(*shared_err, RelayError::DoNotRetry(_));

                    *loop_manager.resource_state.lock().await = ResourceState::Failed(ResourceFailure {
                        retry_wait: if !is_final { Some(retry_in.as_secs()) } else { None },
                        error: shared_err
                    });
                    if is_final {
                        break 'stop;
                    }
                    select! {
                        _ = tokio::time::sleep(retry_in) => {},
                        _ = retry_now_recv.recv() => {},
                        _ = death_recv.recv() => {
                            break 'stop;
                        }
                    };
                    *loop_manager.resource_state.lock().await = ResourceState::Generating;
                    result = loop_manager.resource.generate_unwind_safe().await;
                }
                current_resource = result.unwrap();
                *loop_manager.refreshed_at.lock().await = SystemTime::now();
                *loop_manager.resource_state.lock().await = ResourceState::Generated;
                let _ = generated_send.send(());
                resolve_items(Ok(()), &mut sig_recv, &mut retry_now_recv);
            }
            println!("Resource task closed");
        });

        manager
    }

    pub async fn ensure_not_failed(&self) -> Result<(), RelayError> {
        if let ResourceState::Failed(error) = &*self.resource_state.lock().await {
            return Err(error.error.clone().into())
        }
        Ok(())
    }

    pub async fn request_update(&self) {
        self.retry_signal.send(()).await.unwrap();
    }

    pub async fn refresh(&self) -> Result<(), RelayError> {
        self.refresh_option(false).await
    }
    
    pub async fn refresh_now(&self) -> Result<(), RelayError> {
        self.refresh_option(true).await
    }

    async fn refresh_option(&self, now: bool) -> Result<(), RelayError> {
        let elapsed = self.refreshed_at.lock().await.elapsed().unwrap();
        if elapsed < MAX_RESOURCE_REGEN {
            return Ok(())
        }
        let (send, confirm) = oneshot::channel();
        self.request_retries.send(send).await.unwrap();
        if now {
            self.retry_now_signal.send(()).await.unwrap();
        } else {
            self.retry_signal.send(()).await.unwrap();
        }
        Ok(tokio::time::timeout(MAX_RESOURCE_WAIT, confirm).await.map_err(|_| RelayError::ResourceTimeout)?.unwrap()?)
    }

}