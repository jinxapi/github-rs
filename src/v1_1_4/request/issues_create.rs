
//! Create an issue
//! 
//! Any user with pull access to a repository can create an issue. If [issues are disabled in the repository](https://docs.github.com/articles/disabling-issues/), the API returns a `410 Gone` status.
//! 
//! This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/issues#create-an-issue)

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
    let mut url = String::with_capacity(trimmed.len() + 33);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/issues");
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
impl<'a> TryFrom<&crate::v1_1_4::request::issues_create::body::Json<'a>> for Content<::hyper::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::issues_create::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest")]
impl<'a> TryFrom<&crate::v1_1_4::request::issues_create::body::Json<'a>> for Content<::reqwest::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::issues_create::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest-blocking")]
impl<'a> TryFrom<&crate::v1_1_4::request::issues_create::body::Json<'a>> for Content<::reqwest::blocking::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::issues_create::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

/// Types for body parameter in [`super::issues_create`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The title of the issue.
        pub title: ::serde_json::value::Value,

        /// The contents of the issue.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub body: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Login for the user that this issue should be assigned to. _NOTE: Only users with push access can set the assignee for new issues. The assignee is silently dropped otherwise. **This field is deprecated.**_
        #[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::v1_1_4::support::deserialize_some")]
        pub assignee: ::std::option::Option<::std::option::Option<::std::borrow::Cow<'a, str>>>,

        #[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::v1_1_4::support::deserialize_some")]
        pub milestone: ::std::option::Option<::std::option::Option<::serde_json::value::Value>>,

        /// Labels to associate with this issue. _NOTE: Only users with push access can set labels for new issues. Labels are silently dropped otherwise._
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub labels: ::std::option::Option<::std::borrow::Cow<'a, [::serde_json::value::Value]>>,

        /// Logins for Users to assign to this issue. _NOTE: Only users with push access can set assignees for new issues. Assignees are silently dropped otherwise._
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub assignees: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }
}
