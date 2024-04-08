use std::io::stdout;

use regex::Regex;
use reqwest;
use snm_core::{
    config::cfg::{get_arch, get_os},
    model::SnmError,
    print_warning,
};
use std::time::Duration;

pub async fn check_supported(node_version: &str, node_dist_url: &str) -> Result<(), SnmError> {
    let client = reqwest::Client::new();
    let response = client
        .get(node_dist_url)
        .timeout(Duration::from_secs(1))
        .send()
        .await;

    // if request err, like timeout
    if response.is_err() {
        return Err(SnmError::DownloadFailed {
            download_url: node_dist_url.to_string(),
        });
    }

    let response = response.unwrap();
    let response_status = response.status();

    let os = get_os();
    let arch = get_arch();

    // response is not 200-299
    if !response_status.is_success() {
        return Err(SnmError::UnsupportedPlatform { os, arch });
    }

    let text = response.text().await;

    if text.is_err() {
        return Err(SnmError::DownloadFailed {
            download_url: node_dist_url.to_string(),
        });
    }

    let text = text.unwrap();

    let re = Regex::new(r#"<a\s+href="([^"]+)">[^<]+</a>"#).unwrap();

    let prefix = format!("node-v{}", node_version);

    let os = get_os();
    let arch = get_arch();

    let temp_su = format!("{prefix}-{os}-{arch}");

    let mut supported_list: Vec<String> = vec![];

    let mut is_supported = false;

    for cap in re.captures_iter(&text) {
        let node_item = &cap[1];
        if node_item.contains(&temp_su) {
            is_supported = true;
            break;
        }

        if node_item.starts_with(&prefix) {
            supported_list.push(node_item.to_string())
        }
    }

    if is_supported {
        return Ok(());
    } else {
        let mut out = stdout();
        print_warning!(out, "not support {}", &temp_su);
        let mut out = stdout();
        print_warning!(out, "supported list {:?}", &supported_list);
        return Err(SnmError::UnsupportedPlatform { os, arch });
    }
}
