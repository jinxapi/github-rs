
//! Create a deployment
//! 
//! Deployments offer a few configurable parameters with certain defaults.
//! 
//! The `ref` parameter can be any named branch, tag, or SHA. At GitHub we often deploy branches and verify them
//! before we merge a pull request.
//! 
//! The `environment` parameter allows deployments to be issued to different runtime environments. Teams often have
//! multiple environments for verifying their applications, such as `production`, `staging`, and `qa`. This parameter
//! makes it easier to track which environments have requested deployments. The default environment is `production`.
//! 
//! The `auto_merge` parameter is used to ensure that the requested ref is not behind the repository's default branch. If
//! the ref _is_ behind the default branch for the repository, we will attempt to merge it for you. If the merge succeeds,
//! the API will return a successful merge commit. If merge conflicts prevent the merge from succeeding, the API will
//! return a failure response.
//! 
//! By default, [commit statuses](https://docs.github.com/rest/reference/commits#commit-statuses) for every submitted context must be in a `success`
//! state. The `required_contexts` parameter allows you to specify a subset of contexts that must be `success`, or to
//! specify contexts that have not yet been submitted. You are not required to use commit statuses to deploy. If you do
//! not require any contexts or create any commit statuses, the deployment will always succeed.
//! 
//! The `payload` parameter is available for any extra information that a deployment system might need. It is a JSON text
//! field that will be passed on when a deployment event is dispatched.
//! 
//! The `task` parameter is used by the deployment system to allow different execution paths. In the web world this might
//! be `deploy:migrations` to run schema changes on the system. In the compiled world this could be a flag to compile an
//! application with debugging enabled.
//! 
//! Users with `repo` or `repo_deployment` scopes can create a deployment for a given ref.
//! 
//! #### Merged branch response
//! You will see this response when GitHub automatically merges the base branch into the topic branch instead of creating
//! a deployment. This auto-merge happens when:
//! *   Auto-merge option is enabled in the repository
//! *   Topic branch does not include the latest changes on the base branch, which is `master` in the response example
//! *   There are no merge conflicts
//! 
//! If there are no new commits in the base branch, a new request to create a deployment should give a successful
//! response.
//! 
//! #### Merge conflict response
//! This error happens when the `auto_merge` option is enabled and when the default branch (in this case `master`), can't
//! be merged into the branch that's being deployed (in this case `topic-branch`), due to merge conflicts.
//! 
//! #### Failed commit status checks
//! This error happens when the `required_contexts` parameter indicates that one or more contexts need to have a `success`
//! status for the commit to be deployed, but one or more of the required contexts do not have a state of `success`.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#create-a-deployment)

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
    let mut url = String::with_capacity(trimmed.len() + 38);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/deployments");
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
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_deployment::body::Json<'a>> for Content<::hyper::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_deployment::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_deployment::body::Json<'a>> for Content<::reqwest::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_deployment::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest-blocking")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_create_deployment::body::Json<'a>> for Content<::reqwest::blocking::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_create_deployment::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

/// Types for body parameter in [`super::repos_create_deployment`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The ref to deploy. This can be a branch, tag, or SHA.
        pub r#ref: ::std::borrow::Cow<'a, str>,

        /// Specifies a task to execute (e.g., `deploy` or `deploy:migrations`).
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub task: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Attempts to automatically merge the default branch into the requested ref, if it's behind the default branch.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub auto_merge: ::std::option::Option<bool>,

        /// The [status](https://docs.github.com/rest/reference/commits#commit-statuses) contexts to verify against commit status checks. If you omit this parameter, GitHub verifies all unique contexts before creating a deployment. To bypass checking entirely, pass an empty array. Defaults to all unique contexts.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub required_contexts: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub payload: ::std::option::Option<::serde_json::value::Value>,

        /// Name for the target deployment environment (e.g., `production`, `staging`, `qa`).
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub environment: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Short description of the deployment.
        #[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::v1_1_4::support::deserialize_some")]
        pub description: ::std::option::Option<::std::option::Option<::std::borrow::Cow<'a, str>>>,

        /// Specifies if the given environment is specific to the deployment and will no longer exist at some point in the future. Default: `false`
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub transient_environment: ::std::option::Option<bool>,

        /// Specifies if the given environment is one that end-users directly interact with. Default: `true` when `environment` is `production` and `false` otherwise.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub production_environment: ::std::option::Option<bool>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }
}
