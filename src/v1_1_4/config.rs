use std::borrow::Cow;
use std::sync::Arc;

pub enum Authentication {
    None,
    AccessToken(Arc<::authentic::credential::TokenCredential>),
    Basic(Arc<::authentic::credential::UsernamePasswordCredential>),
    JWT(Arc<::authentic::credential::JsonWebTokenCredential>),
}

pub struct Configuration {
    pub authentication: Authentication,
    pub base_url: Cow<'static, str>,
    pub user_agent: Cow<'static, str>,
    pub accept: Option<Cow<'static, str>>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            authentication: Authentication::None,
            base_url: Cow::default(),
            user_agent: "jinxapi-github/0.1.0".into(),
            accept: Some("application/vnd.github.v3+json".into()),
        }
    }
}