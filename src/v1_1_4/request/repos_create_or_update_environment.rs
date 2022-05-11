//! Create or update an environment
//! 
//! Create or update an environment with protection rules, such as required reviewers. For more information about environment protection rules, see "[Environments](/actions/reference/environments#environment-protection-rules)."
//! 
//! **Note:** Although you can use this operation to specify that only branches that match specified name patterns can deploy to this environment, you must use the UI to set the name patterns. For more information, see "[Environments](/actions/reference/environments#deployment-branches)."
//! 
//! **Note:** To create or update secrets for an environment, see "[Secrets](/rest/reference/actions#secrets)."
//! 
//! You must authenticate using an access token with the repo scope to use this endpoint.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#create-or-update-an-environment)

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
    p_environment_name: &str,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 41);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/environments/");
    ::querylizer::Simple::extend(&mut url, &p_environment_name, false, &::querylizer::encode_path)?;
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_environment_name: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_environment_name,
    )?;
    let mut builder = ::http::request::Request::put(url);
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
    p_environment_name: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_environment_name,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::Request::new(::reqwest::Method::PUT, reqwest_url);
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
    p_environment_name: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_environment_name,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::blocking::Request::new(::reqwest::Method::PUT, reqwest_url);
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

/// Types for body parameter in [`super::repos_create_or_update_environment`]
pub mod body {
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The amount of time to delay a job after the job is initially triggered. The time (in minutes) must be an integer between 0 and 43,200 (30 days).
        /// 
        /// # Example
        /// 
        /// ```json
        /// 30
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub wait_timer: ::std::option::Option<i64>,

        /// The people or teams that may review jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed.
        #[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::v1_1_4::support::deserialize_some")]
        pub reviewers: ::std::option::Option<::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::repos_create_or_update_environment::body::json::Reviewers<'a>]>>>,

        #[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::v1_1_4::support::deserialize_some")]
        pub deployment_branch_policy: ::std::option::Option<::std::option::Option<crate::v1_1_4::schema::DeploymentBranchPolicy<'a>>>,
    }

    /// Types for fields in [`Json`]
    pub mod json {
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Reviewers<'a> {
            /// The type of reviewer. Must be one of: `User` or `Team`
            /// 
            /// # Example
            /// 
            /// ```json
            /// "User"
            /// ```
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub r#type: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            /// The id of the user or team who can review the deployment
            /// 
            /// # Example
            /// 
            /// ```json
            /// 4532992
            /// ```
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub id: ::std::option::Option<i64>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }
    }

    #[cfg(feature = "hyper")]
    impl<'a> TryFrom<&::std::option::Option<crate::v1_1_4::request::repos_create_or_update_environment::body::Json<'a>>> for super::Content<::hyper::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &::std::option::Option<crate::v1_1_4::request::repos_create_or_update_environment::body::Json<'a>>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }

    #[cfg(feature = "reqwest")]
    impl<'a> TryFrom<&::std::option::Option<crate::v1_1_4::request::repos_create_or_update_environment::body::Json<'a>>> for super::Content<::reqwest::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &::std::option::Option<crate::v1_1_4::request::repos_create_or_update_environment::body::Json<'a>>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }

    #[cfg(feature = "reqwest-blocking")]
    impl<'a> TryFrom<&::std::option::Option<crate::v1_1_4::request::repos_create_or_update_environment::body::Json<'a>>> for super::Content<::reqwest::blocking::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &::std::option::Option<crate::v1_1_4::request::repos_create_or_update_environment::body::Json<'a>>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }
}
