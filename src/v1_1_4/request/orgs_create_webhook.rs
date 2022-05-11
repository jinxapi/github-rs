//! Create an organization webhook
//! 
//! Here's how you can create a hook that posts payloads in JSON format:
//! 
//! [API method documentation](https://docs.github.com/rest/reference/orgs#create-an-organization-webhook)

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

fn url_string(
    base_url: &str,
    p_org: &str,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 29);
    url.push_str(trimmed);
    url.push_str("/orgs/");
    ::querylizer::Simple::extend(&mut url, &p_org, false, &::querylizer::encode_path)?;
    url.push_str("/hooks");
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_org: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
    )?;
    let mut builder = ::http::request::Request::post(url);
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
    p_org: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::Request::new(::reqwest::Method::POST, reqwest_url);
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
    p_org: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::blocking::Request::new(::reqwest::Method::POST, reqwest_url);
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

/// Types for body parameter in [`super::orgs_create_webhook`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// Must be passed as "web".
        pub name: ::std::borrow::Cow<'a, str>,

        pub config: crate::v1_1_4::request::orgs_create_webhook::body::json::Config<'a>,

        /// Determines what [events](https://docs.github.com/webhooks/event-payloads) the hook is triggered for.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub events: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

        /// Determines if notifications are sent when the webhook is triggered. Set to `true` to send notifications.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub active: ::std::option::Option<bool>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }

    /// Types for fields in [`Json`]
    pub mod json {
        /// Key/value pairs to provide settings for this webhook. [These are defined below](https://docs.github.com/rest/reference/orgs#create-hook-config-params).
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Config<'a> {
            /// The URL to which the payloads will be delivered.
            /// 
            /// # Example
            /// 
            /// ```json
            /// "https://example.com/webhook"
            /// ```
            pub url: ::std::borrow::Cow<'a, str>,

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

            /// # Example
            /// 
            /// ```json
            /// "\"kdaigle\""
            /// ```
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub username: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            /// # Example
            /// 
            /// ```json
            /// "\"password\""
            /// ```
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub password: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }
    }

    #[cfg(feature = "hyper")]
    impl<'a> TryFrom<&Json<'a>> for super::Content<::hyper::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &Json<'a>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }

    #[cfg(feature = "reqwest")]
    impl<'a> TryFrom<&Json<'a>> for super::Content<::reqwest::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &Json<'a>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }

    #[cfg(feature = "reqwest-blocking")]
    impl<'a> TryFrom<&Json<'a>> for super::Content<::reqwest::blocking::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &Json<'a>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }
}
