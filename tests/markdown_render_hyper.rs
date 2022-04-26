use std::sync::Arc;

use authentic::credential::TokenCredential;
use http::StatusCode;
use hyper::body::HttpBody as _;
use hyper::Client;
use hyper_tls::HttpsConnector;
use jinxapi_github::v1_1_4 as github_api;

#[tokio::test]
async fn test_markdown_render_json(
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _ = env_logger::builder().is_test(true).try_init();

    let https = HttpsConnector::new();
    let hyper_client = Client::builder().build::<_, ::hyper::Body>(https);

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

    let github = github_api::hyper::Caller::new(hyper_client, config, tokio::time::sleep);

    let json = github_api::request::markdown_render::body::Json {
        text: "*Hello*, World!\n\nJinx!".into(),
        ..Default::default()
    };

    let mut response = github.markdown_render(&json).await?;

    assert_eq!(response.status(), StatusCode::OK);

    let mut actual: Vec<u8> = Vec::new();
    while let Some(chunk) = response.body_mut().data().await {
        let bytes = chunk?;
        actual.extend(&bytes[..]);
    }
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
