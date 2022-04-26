#![cfg(feature = "reqwest_blocking")]

use std::sync::Arc;

use authentic::credential::TokenCredential;
use http::StatusCode;
use jinxapi_github::v1_1_4 as github_api;
use reqwest::blocking::Client;

#[test]
fn test_markdown_render_json() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::builder().is_test(true).try_init();

    let client = Client::new();

    // `markdown_render` works without authentication. Allow it to be used with
    // authentication by providing an optional access token.
    let (config, authenticated) = match std::env::var("GITHUB_TOKEN") {
        Ok(token) => (
            github_api::config::Configuration {
                authentication: github_api::config::Authentication::AccessToken(Arc::new(
                    TokenCredential::new(token.into_bytes()),
                )),
                ..Default::default()
            },
            true,
        ),
        Err(_) => (github_api::config::Configuration::default(), false),
    };

    let github = github_api::reqwest::blocking::Caller::new(client, config, std::thread::sleep);

    let json = github_api::request::markdown_render::body::Json {
        text: "*Hello*, World!\n\nJinx!".into(),
        ..Default::default()
    };

    let mut response = github.markdown_render(&json)?;

    assert_eq!(response.status(), StatusCode::OK);

    let mut actual: Vec<u8> = Vec::new();
    response.copy_to(&mut actual)?;
    let expected = "<p><em>Hello</em>, World!</p>\n<p>Jinx!</p>\n";
    assert_eq!(std::str::from_utf8(&actual)?, expected);

    // Check that authentication worked as expected. Unauthenticated requests get a low
    // rate limit (60). Authenticated requests get a high rate limit (6000).
    let rate_limit: u32 = response
        .headers()
        .get("x-ratelimit-limit")
        .unwrap()
        .to_str()?
        .parse()?;
    if authenticated {
        assert!(rate_limit > 1000);
    } else {
        assert!(rate_limit < 1000);
    }

    Ok(())
}
