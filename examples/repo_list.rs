use std::sync::Arc;

use authentic::credential::TokenCredential;
use hyper::body::HttpBody as _;
use hyper::StatusCode;
use hyper_tls::HttpsConnector;
use jinxapi_github::v1_1_4 as github_api;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let https = HttpsConnector::new();
    let hyper_client = hyper::Client::builder().build::<_, ::hyper::Body>(https);

    let owner = std::env::var("GITHUB_USER")?;
    let token = std::env::var("GITHUB_TOKEN")?;
    let credential = TokenCredential::new(token.into_bytes());

    let config = github_api::config::Configuration {
        authentication: github_api::config::Authentication::AccessToken(Arc::new(credential)),
        ..Default::default()
    };

    let github = github_api::hyper::Caller::new(hyper_client, config, tokio::time::sleep);

    for page in 1.. {
        let mut response = github
            .repos_list_for_user(
                &owner,
                None,
                &jinxapi_github::types::Sort::Default,
                None,
                Some(page),
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);

        let mut output: Vec<u8> = Vec::new();
        while let Some(chunk) = response.body_mut().data().await {
            let bytes = chunk?;
            output.extend(&bytes[..]);
        }
        let v: Value = serde_json::from_slice(&output)?;
        let repos = v.as_array().unwrap();
        if repos.is_empty() {
            break;
        }
        for repo in repos {
            if let Some(fork) = repo.get("fork") {
                let full_name = repo.get("full_name").unwrap().as_str().unwrap();
                print!("{}", full_name);
                if fork.as_bool().unwrap() {
                    print!(" (fork)");
                }
                let description = repo.get("description").unwrap().as_str().unwrap_or("");
                if description.is_empty() {
                    println!();
                } else {
                    println!(": {}", description);
                }
            }
        }
    }

    Ok(())
}
