use super::Request as RuneRequest;
use crate::app_response::AppResult;
use crate::constant::TEST_NET_RUNES_RPC;
use salvo::oapi::endpoint;
use salvo::oapi::extract::*;

/// Get the current block height
#[endpoint]
pub async fn blocktime() -> AppResult<String> {
    let body = reqwest::get(format!("{}/blocktime", TEST_NET_RUNES_RPC))
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(body)
}

#[endpoint]
pub async fn tx(txid: JsonBody<RuneRequest<String>>) -> AppResult<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/tx/{}", TEST_NET_RUNES_RPC, txid.0.request))
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    // Check if the response is successful
    if response.status().is_success() {
        let body = response.text().await.map_err(|e| anyhow::anyhow!(e))?;

        // Parse the response text as JSON
        Ok(serde_json::from_str::<serde_json::Value>(&body)
            .map_err(|e| anyhow::anyhow!(e))
            .map(|v| v.to_string())?)
    } else {
        // Handle non-successful response
        Err(anyhow::anyhow!(response.status()).into())
    }
}
