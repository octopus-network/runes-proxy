use super::Request as RuneRequest;
use crate::app_response::AppResult;
use crate::constant::TEST_NET_RUNES_RPC;
use salvo::oapi::endpoint;
use salvo::oapi::extract::*;

///
#[endpoint]
pub async fn block(query: JsonBody<RuneRequest<String>>) -> AppResult<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/block/{}", TEST_NET_RUNES_RPC, query.0.request))
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

/// Get the current block count
#[endpoint]
pub async fn blockcount() -> AppResult<String> {
    let body = reqwest::get(format!("{}/blockcount", TEST_NET_RUNES_RPC))
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(body)
}

/// Get the current block hash
#[endpoint]
pub async fn blockhash() -> AppResult<String> {
    let body = reqwest::get(format!("{}/r/blockhash", TEST_NET_RUNES_RPC))
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(body)
}

/// Get the current block hash
#[endpoint]
pub async fn blockhash_by_height(height: JsonBody<RuneRequest<String>>) -> AppResult<String> {
    let body = reqwest::get(format!(
        "{}/blockhash/{}",
        TEST_NET_RUNES_RPC, height.0.request
    ))
    .await
    .map_err(|e| anyhow::anyhow!(e))?
    .text()
    .await
    .map_err(|e| anyhow::anyhow!(e))?;

    Ok(body)
}

/// Get the current block hash
#[endpoint]
pub async fn blockheight() -> AppResult<String> {
    let body = reqwest::get(format!("{}/r/blockheight", TEST_NET_RUNES_RPC))
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(body)
}

/// Get the current block hash
#[endpoint]
pub async fn blocks() -> AppResult<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/blocks", TEST_NET_RUNES_RPC))
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

/// Get the current block height
#[endpoint]
pub async fn clock() -> AppResult<String> {
    let body = reqwest::get(format!("{}/clock", TEST_NET_RUNES_RPC))
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(body)
}

/// Get the current block hash
#[endpoint]
pub async fn collections() -> AppResult<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/collections", TEST_NET_RUNES_RPC))
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

/// Get the current block hash
#[endpoint]
pub async fn inscriptions() -> AppResult<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/inscriptions", TEST_NET_RUNES_RPC))
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

#[endpoint]
pub async fn rune(rune_id: JsonBody<RuneRequest<String>>) -> AppResult<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "{}/runes/{}",
            TEST_NET_RUNES_RPC, rune_id.0.request
        ))
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

#[endpoint]
pub async fn runes() -> AppResult<String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/runes", TEST_NET_RUNES_RPC))
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

/// Get the current block height
#[endpoint]
pub async fn status() -> AppResult<String> {
    let body = reqwest::get(format!("{}/status", TEST_NET_RUNES_RPC))
        .await
        .map_err(|e| anyhow::anyhow!(e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(body)
}
