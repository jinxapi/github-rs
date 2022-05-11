//! Update a repository
//! 
//! **Note**: To edit a repository's topics, use the [Replace all repository topics](https://docs.github.com/rest/reference/repos#replace-all-repository-topics) endpoint.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos/#update-a-repository)

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
    let mut url = String::with_capacity(trimmed.len() + 26);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
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

/// Types for body parameter in [`super::repos_update`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The name of the repository.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub name: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// A short description of the repository.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub description: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// A URL with more information about the repository.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub homepage: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Either `true` to make the repository private or `false` to make it public. Default: `false`.  
        /// **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://docs.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private. **Note**: You will get a `422` error if the organization restricts [changing repository visibility](https://docs.github.com/articles/repository-permission-levels-for-an-organization#changing-the-visibility-of-repositories) to organization owners and a non-owner tries to change the value of private.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub private: ::std::option::Option<bool>,

        /// Can be `public` or `private`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `visibility` can also be `internal`."
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub visibility: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        #[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::v1_1_4::support::deserialize_some")]
        pub security_and_analysis: ::std::option::Option<::std::option::Option<crate::v1_1_4::request::repos_update::body::json::SecurityAndAnalysis<'a>>>,

        /// Either `true` to enable issues for this repository or `false` to disable them.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_issues: ::std::option::Option<bool>,

        /// Either `true` to enable projects for this repository or `false` to disable them. **Note:** If you're creating a repository in an organization that has disabled repository projects, the default is `false`, and if you pass `true`, the API returns an error.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_projects: ::std::option::Option<bool>,

        /// Either `true` to enable the wiki for this repository or `false` to disable it.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_wiki: ::std::option::Option<bool>,

        /// Either `true` to make this repo available as a template repository or `false` to prevent it.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub is_template: ::std::option::Option<bool>,

        /// Updates the default branch for this repository.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub default_branch: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Either `true` to allow squash-merging pull requests, or `false` to prevent squash-merging.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_squash_merge: ::std::option::Option<bool>,

        /// Either `true` to allow merging pull requests with a merge commit, or `false` to prevent merging pull requests with merge commits.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_merge_commit: ::std::option::Option<bool>,

        /// Either `true` to allow rebase-merging pull requests, or `false` to prevent rebase-merging.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_rebase_merge: ::std::option::Option<bool>,

        /// Either `true` to allow auto-merge on pull requests, or `false` to disallow auto-merge.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_auto_merge: ::std::option::Option<bool>,

        /// Either `true` to allow automatically deleting head branches when pull requests are merged, or `false` to prevent automatic deletion.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub delete_branch_on_merge: ::std::option::Option<bool>,

        /// `true` to archive this repository. **Note**: You cannot unarchive repositories through the API.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub archived: ::std::option::Option<bool>,

        /// Either `true` to allow private forks, or `false` to prevent private forks.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_forking: ::std::option::Option<bool>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }

    /// Types for fields in [`Json`]
    pub mod json {
        /// Specify which security and analysis features to enable or disable. For example, to enable GitHub Advanced Security, use this data in the body of the PATCH request: `{"security_and_analysis": {"advanced_security": {"status": "enabled"}}}`. If you have admin permissions for a private repository covered by an Advanced Security license, you can check which security and analysis features are currently enabled by using a `GET /repos/{owner}/{repo}` request.
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct SecurityAndAnalysis<'a> {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub advanced_security: ::std::option::Option<crate::v1_1_4::request::repos_update::body::json::security_and_analysis::AdvancedSecurity<'a>>,

            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub secret_scanning: ::std::option::Option<crate::v1_1_4::request::repos_update::body::json::security_and_analysis::SecretScanning<'a>>,

            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub secret_scanning_push_protection: ::std::option::Option<crate::v1_1_4::request::repos_update::body::json::security_and_analysis::SecretScanningPushProtection<'a>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }

        /// Types for fields in [`SecurityAndAnalysis`]
        pub mod security_and_analysis {
            /// Use the `status` property to enable or disable GitHub Advanced Security for this repository. For more information, see "[About GitHub Advanced Security](/github/getting-started-with-github/learning-about-github/about-github-advanced-security)."
            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct AdvancedSecurity<'a> {
                /// Can be `enabled` or `disabled`.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub status: ::std::option::Option<::std::borrow::Cow<'a, str>>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }

            /// Use the `status` property to enable or disable secret scanning for this repository. For more information, see "[About secret scanning](/code-security/secret-security/about-secret-scanning)."
            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct SecretScanning<'a> {
                /// Can be `enabled` or `disabled`.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub status: ::std::option::Option<::std::borrow::Cow<'a, str>>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }

            /// Use the `status` property to enable or disable secret scanning push protection for this repository. For more information, see "[Protecting pushes with secret scanning](/code-security/secret-scanning/protecting-pushes-with-secret-scanning)."
            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct SecretScanningPushProtection<'a> {
                /// Can be `enabled` or `disabled`.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub status: ::std::option::Option<::std::borrow::Cow<'a, str>>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }
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
