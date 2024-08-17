
mod c;
mod error;
mod nac;
mod relay;
mod util;

use std::sync::Arc;

use base64::engine::general_purpose;
use base64::Engine;
use nac::generate_validation_data;
use relay::{Relay, RelayResource, RelayState};
use serde::{Deserialize, Serialize};
use tokio::{fs, sync::broadcast};


pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

#[derive(Serialize, Deserialize)]
struct RelayConfig {
    url: String,
    state: Option<RelayState>,
}

impl RelayConfig {
    async fn from_relay(relay: &Relay) -> RelayConfig {
        RelayConfig {
            url: relay.url.lock().await.clone(),
            state: relay.state.lock().await.clone(),
        }
    }
}

#[tokio::main]
async fn main() {
    let config_path = "config.json";

    let mut config: Option<RelayConfig> = None;
    if let Ok(read) = fs::read_to_string(config_path).await {
        if let Ok(item) = serde_json::from_str(&read) {
            config = Some(item);
        }
    }

    let url = config.as_ref().map(|i| i.url.clone()).unwrap_or_else(|| "wss://registration-relay.beeper.com/api/v1/provider".to_string());
    let relay = RelayResource::new(url, config.as_ref().and_then(|i| i.state.clone()));

    let mut to_refresh = relay.generated_signal.subscribe();
    let reconn_conn = Arc::downgrade(&relay);
    tokio::spawn(async move {
        loop {
            match to_refresh.recv().await {
                Ok(()) => {
                    let Some(conn) = reconn_conn.upgrade() else { break };
                    // update keys
                    let state = RelayConfig::from_relay(&conn).await;
                    std::fs::write(config_path, serde_json::to_string(&state).unwrap()).unwrap();
                },
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });


    

    loop { }
}
