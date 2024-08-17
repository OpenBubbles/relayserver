
use std::io::Cursor;

use crate::c::{nac_init_rs, nac_key_establishment_rs, nac_sign_rs};
use crate::error::RelayError;
use plist::{Data, Error};
use serde::{Serialize, Deserialize};


#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct SessionInfoRequest {
    session_info_request: Data,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SessionInfoResponse {
    session_info: Data,
}

#[derive(Deserialize)]
struct CertsResponse {
    cert: Data,
}

pub fn plist_to_buf<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let writer = Cursor::new(&mut buf);
    plist::to_writer_xml(writer, &value)?;
    Ok(buf)
}

pub async fn generate_validation_data() -> Result<Vec<u8>, RelayError> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .use_rustls_tls()
        .build()
        .unwrap();

    let key = client.get("http://static.ess.apple.com/identity/validation/cert-1.0.plist")
        .send().await?;
    let response: CertsResponse = plist::from_bytes(&key.bytes().await?)?;
    let certs: Vec<u8> = response.cert.into();

    let mut output_req = vec![];
    let ctx = nac_init_rs(&certs, &mut output_req)?;

    let init = SessionInfoRequest {
        session_info_request: output_req.into()
    };

    let info = plist_to_buf(&init)?;
    let activation = client.post("https://identity.ess.apple.com/WebObjects/TDIdentityService.woa/wa/initializeValidation")
        .body(info)
        .send().await?;

    let response: SessionInfoResponse = plist::from_bytes(&activation.bytes().await?)?;
    let output: Vec<u8> = response.session_info.into();
    nac_key_establishment_rs(ctx, &output)?;

    
    Ok(nac_sign_rs(ctx, &[])?)
}