
//! Create a repository for the authenticated user
//! 
//! Creates a new repository for the authenticated user.
//! 
//! **OAuth scope requirements**
//! 
//! When using [OAuth](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), authorizations must include:
//! 
//! *   `public_repo` scope or `repo` scope to create a public repository. Note: For GitHub AE, use `repo` scope to create an internal repository.
//! *   `repo` scope to create a private repository.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#create-a-repository-for-the-authenticated-user)

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
    let default_url = concat!("https://api.github.com", "/user/repos");
    let url = if base_url.is_empty() {
        ::http::uri::Uri::from_static(default_url)
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 11);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        url.try_into().map_err(::http::Error::from)?
    };
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
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/user/repos");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 11);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
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
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/user/repos");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 11);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
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
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_for_authenticated_user::body::Json<'a>> for Content<::hyper::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_for_authenticated_user::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_for_authenticated_user::body::Json<'a>> for Content<::reqwest::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_for_authenticated_user::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest-blocking")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_for_authenticated_user::body::Json<'a>> for Content<::reqwest::blocking::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_for_authenticated_user::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

/// Types for body parameter in [`super::repos_create_for_authenticated_user`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The name of the repository.
        /// 
        /// # Example
        /// 
        /// ```json
        /// "Team Environment"
        /// ```
        pub name: ::std::borrow::Cow<'a, str>,

        /// A short description of the repository.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub description: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// A URL with more information about the repository.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub homepage: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Whether the repository is private.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub private: ::std::option::Option<bool>,

        /// Whether issues are enabled.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_issues: ::std::option::Option<bool>,

        /// Whether projects are enabled.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_projects: ::std::option::Option<bool>,

        /// Whether the wiki is enabled.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_wiki: ::std::option::Option<bool>,

        /// The id of the team that will be granted access to this repository. This is only valid when creating a repository in an organization.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub team_id: ::std::option::Option<i64>,

        /// Whether the repository is initialized with a minimal README.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub auto_init: ::std::option::Option<bool>,

        /// The desired language or platform to apply to the .gitignore.
        /// 
        /// # Example
        /// 
        /// ```json
        /// "Haskell"
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub gitignore_template: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The license keyword of the open source license for this repository.
        /// 
        /// # Example
        /// 
        /// ```json
        /// "mit"
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub license_template: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Whether to allow squash merges for pull requests.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_squash_merge: ::std::option::Option<bool>,

        /// Whether to allow merge commits for pull requests.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_merge_commit: ::std::option::Option<bool>,

        /// Whether to allow rebase merges for pull requests.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_rebase_merge: ::std::option::Option<bool>,

        /// Whether to allow Auto-merge to be used on pull requests.
        /// 
        /// # Example
        /// 
        /// ```json
        /// false
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_auto_merge: ::std::option::Option<bool>,

        /// Whether to delete head branches when pull requests are merged
        /// 
        /// # Example
        /// 
        /// ```json
        /// false
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub delete_branch_on_merge: ::std::option::Option<bool>,

        /// Whether downloads are enabled.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_downloads: ::std::option::Option<bool>,

        /// Whether this repository acts as a template that can be used to generate new repositories.
        /// 
        /// # Example
        /// 
        /// ```json
        /// true
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub is_template: ::std::option::Option<bool>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }
}
