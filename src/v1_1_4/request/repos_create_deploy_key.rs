
//! Create a deploy key
//! 
//! You can create a read-only deploy key.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#create-a-deploy-key)

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
    p_owner: &str,
    p_repo: &str,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 31);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/keys");
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
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
    p_owner: &str,
    p_repo: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
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
    p_owner: &str,
    p_repo: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
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

#[cfg(feature = "hyper")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_deploy_key::body::Json<'a>> for Content<::hyper::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_deploy_key::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_deploy_key::body::Json<'a>> for Content<::reqwest::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_deploy_key::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest-blocking")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_deploy_key::body::Json<'a>> for Content<::reqwest::blocking::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_deploy_key::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

/// Types for body parameter in [`super::repos_create_deploy_key`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// A name for the key.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub title: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The contents of the key.
        pub key: ::std::borrow::Cow<'a, str>,

        /// If `true`, the key will only be able to read repository contents. Otherwise, the key will be able to read and write.  
        ///   
        /// Deploy keys with write access can perform the same actions as an organization member with admin access, or a collaborator on a personal repository. For more information, see "[Repository permission levels for an organization](https://help.github.com/articles/repository-permission-levels-for-an-organization/)" and "[Permission levels for a user account repository](https://help.github.com/articles/permission-levels-for-a-user-account-repository/)."
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub read_only: ::std::option::Option<bool>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }
}