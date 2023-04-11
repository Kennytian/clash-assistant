use reqwest::get;
use serde_json::{from_str, Value};
use std::collections::HashMap;

#[derive(serde::Serialize)]
pub struct IPInfo {
    origin: String,
    server: String,
}

#[tauri::command]
pub async fn get_ip() -> Result<IPInfo, String> {
    let network_err_msg: &str = "网络错误";
    let domain_addr: &str = "https://httpbin.org/ip";

    let resp_text: String = get(domain_addr)
        .await
        .map_err(|_| network_err_msg)?
        .text()
        .await
        .map_err(|_| network_err_msg)?;
    println!("{}", resp_text);

    let raw_info: Value = from_str::<Value>(&resp_text).map_err(|_| "解析错误")?;

    let origin: String = raw_info["origin"].as_str().unwrap_or_default().to_string();
    let server: String = "Rust".into();

    if origin.is_empty() {
        return Err("无效的请求".into());
    }

    Ok(IPInfo { origin, server })
}

#[tauri::command]
pub async fn get_ip2() -> Result<IPInfo, String> {
    let network_err_msg: &str = "网络错误";
    let domain_addr: &str = "https://httpbin.org/ip";

    let resp_text: HashMap<String, String> = get(domain_addr)
        .await
        .map_err(|_| network_err_msg)?
        .json()
        .await
        .map_err(|_| network_err_msg)?;

    let origin: String = resp_text["origin"].as_str().to_string();

    Ok(IPInfo {
        origin,
        server: "Node.js".into(),
    })
}
