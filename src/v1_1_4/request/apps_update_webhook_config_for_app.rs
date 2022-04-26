
//! Update a webhook configuration for an app
//! 
//! Updates the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
//! 
//! You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/apps#update-a-webhook-configuration-for-an-app)

pub struct Content<Body>
{
    body: Body,
    content_type_value: Option<::std::borrow::Cow<'static, [u8]>>,
}

impl<Body> Content<Body> {
    pub fn new(body: Body) -> Self {
        Self { body, content_type_value: None }
    }

    #[must_use]
    pub fn with_content_type(mut self, content_type: impl Into<::std::borrow::Cow<'static, [u8]>>) -> Self {
        self.content_type_value = Some(content_type.into());
        self
    }

    fn content_type(&self) -> Option<&[u8]> {
        self.content_type_value.as_deref()
    }

    fn into_body(self) -> Body {
        self.body
    }
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/app/hook/config");
    let url = if base_url.is_empty() {
        ::http::uri::Uri::from_static(default_url)
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 16);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        url.try_into().map_err(::http::Error::from)?
    };
    let mut builder = ::http::request::Request::patch(url);
    builder = builder.header(
        "User-Agent",
        &::querylizer::Simple::to_string(&h_user_agent, false, &::querylizer::passthrough)?
    );
    if let Some(value) = &h_accept {
        builder = builder.header(
            "Accept",
            &::querylizer::Simple::to_string(value, false, &::querylizer::passthrough)?
        );
    }
    Ok(builder)
}

#[cfg(feature = "hyper")]
pub fn hyper_request(
    mut builder: ::http::request::Builder,
    content: Content<::hyper::Body>,
) -> Result<::http::request::Request<::hyper::Body>, crate::v1_1_4::ApiError>
{
    if let Some(content_type) = content.content_type() {
        builder = builder.header(::http::header::CONTENT_TYPE, content_type);
    }
    Ok(builder.body(content.into_body())?)
}

#[cfg(feature = "hyper")]
impl From<::hyper::Body> for Content<::hyper::Body> {
    fn from(body: ::hyper::Body) -> Self {
        Self::new(body)
    }
}

#[cfg(feature = "reqwest")]
pub fn reqwest_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/app/hook/config");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 16);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
    let mut request = ::reqwest::Request::new(::reqwest::Method::PATCH, reqwest_url);
    let headers = request.headers_mut();
    headers.append(
        "User-Agent",
        ::querylizer::Simple::to_string(&h_user_agent, false, &::querylizer::passthrough)?.try_into()?
    );
    if let Some(value) = &h_accept {
        headers.append(
            "Accept",
            ::querylizer::Simple::to_string(value, false, &::querylizer::passthrough)?.try_into()?
        );
    }
    Ok(request)
}

#[cfg(feature = "reqwest")]
pub fn reqwest_request(
    mut builder: ::reqwest::Request,
    content: Content<::reqwest::Body>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    if let Some(content_type) = content.content_type() {
        builder.headers_mut().append(
            ::reqwest::header::HeaderName::from_static("content-type"),
            ::reqwest::header::HeaderValue::try_from(content_type)?,
        );
    }
    *builder.body_mut() = Some(content.into_body());
    Ok(builder)
}

#[cfg(feature = "reqwest")]
impl From<::reqwest::Body> for Content<::reqwest::Body> {
    fn from(body: ::reqwest::Body) -> Self {
        Self::new(body)
    }
}

#[cfg(feature = "reqwest-blocking")]
pub fn reqwest_blocking_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/app/hook/config");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 16);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
    let mut request = ::reqwest::blocking::Request::new(::reqwest::Method::PATCH, reqwest_url);
    let headers = request.headers_mut();
    headers.append(
        "User-Agent",
        ::querylizer::Simple::to_string(&h_user_agent, false, &::querylizer::passthrough)?.try_into()?
    );
    if let Some(value) = &h_accept {
        headers.append(
            "Accept",
            ::querylizer::Simple::to_string(value, false, &::querylizer::passthrough)?.try_into()?
        );
    }
    Ok(request)
}

#[cfg(feature = "reqwest-blocking")]
pub fn reqwest_blocking_request(
    mut builder: ::reqwest::blocking::Request,
    content: Content<::reqwest::blocking::Body>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    if let Some(content_type) = content.content_type() {
        builder.headers_mut().append(
            ::reqwest::header::HeaderName::from_static("content-type"),
            ::reqwest::header::HeaderValue::try_from(content_type)?,
        );
    }
    *builder.body_mut() = Some(content.into_body());
    Ok(builder)
}

#[cfg(feature = "reqwest-blocking")]
impl From<::reqwest::blocking::Body> for Content<::reqwest::blocking::Body> {
    fn from(body: ::reqwest::blocking::Body) -> Self {
        Self::new(body)
    }
}

#[cfg(feature = "hyper")]
impl<'a> TryFrom<&crate::v1_1_4::request::apps_update_webhook_config_for_app::body::Json<'a>> for Content<::hyper::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::apps_update_webhook_config_for_app::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest")]
impl<'a> TryFrom<&crate::v1_1_4::request::apps_update_webhook_config_for_app::body::Json<'a>> for Content<::reqwest::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::apps_update_webhook_config_for_app::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest-blocking")]
impl<'a> TryFrom<&crate::v1_1_4::request::apps_update_webhook_config_for_app::body::Json<'a>> for Content<::reqwest::blocking::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::apps_update_webhook_config_for_app::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

/// Types for body parameter in [`super::apps_update_webhook_config_for_app`]
pub mod body {
    /// # Example
    /// 
    /// ```json
    /// {
    ///   "content_type": "json",
    ///   "insecure_ssl": "0",
    ///   "secret": "********",
    ///   "url": "https://example.com/webhook"
    /// }
    /// ```
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The URL to which the payloads will be delivered.
        /// 
        /// # Example
        /// 
        /// ```json
        /// "https://example.com/webhook"
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub url: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The media type used to serialize the payloads. Supported values include `json` and `form`. The default is `form`.
        /// 
        /// # Example
        /// 
        /// ```json
        /// "\"json\""
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub content_type: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// If provided, the `secret` will be used as the `key` to generate the HMAC hex digest value for [delivery signature headers](https://docs.github.com/webhooks/event-payloads/#delivery-headers).
        /// 
        /// # Example
        /// 
        /// ```json
        /// "\"********\""
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub secret: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub insecure_ssl: ::std::option::Option<::serde_json::value::Value>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }
}
