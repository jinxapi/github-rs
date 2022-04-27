
//! Update branch protection
//! 
//! Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
//! 
//! Protecting a branch requires admin or owner permissions to the repository.
//! 
//! **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
//! 
//! **Note**: The list of users, apps, and teams in total is limited to 100 items.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#update-branch-protection)

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
    p_branch: &str,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 48);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/branches/");
    ::querylizer::Simple::extend(&mut url, &p_branch, false, &::querylizer::encode_path)?;
    url.push_str("/protection");
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_branch: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_branch,
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
    p_branch: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_branch,
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
    p_branch: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_branch,
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

#[cfg(feature = "hyper")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_update_branch_protection::body::Json<'a>> for Content<::hyper::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_update_branch_protection::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_update_branch_protection::body::Json<'a>> for Content<::reqwest::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_update_branch_protection::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest-blocking")]
impl<'a> TryFrom<&crate::v1_1_4::request::repos_update_branch_protection::body::Json<'a>> for Content<::reqwest::blocking::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::repos_update_branch_protection::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

/// Types for body parameter in [`super::repos_update_branch_protection`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        pub required_status_checks: ::std::option::Option<crate::v1_1_4::request::repos_update_branch_protection::body::json::RequiredStatusChecks<'a>>,

        /// Enforce all configured restrictions for administrators. Set to `true` to enforce required status checks for repository administrators. Set to `null` to disable.
        pub enforce_admins: ::std::option::Option<bool>,

        pub required_pull_request_reviews: ::std::option::Option<crate::v1_1_4::request::repos_update_branch_protection::body::json::RequiredPullRequestReviews<'a>>,

        pub restrictions: ::std::option::Option<crate::v1_1_4::request::repos_update_branch_protection::body::json::Restrictions<'a>>,

        /// Enforces a linear commit Git history, which prevents anyone from pushing merge commits to a branch. Set to `true` to enforce a linear commit history. Set to `false` to disable a linear commit Git history. Your repository must allow squash merging or rebase merging before you can enable a linear commit history. Default: `false`. For more information, see "[Requiring a linear commit history](https://docs.github.com/github/administering-a-repository/requiring-a-linear-commit-history)" in the GitHub Help documentation.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub required_linear_history: ::std::option::Option<bool>,

        /// Permits force pushes to the protected branch by anyone with write access to the repository. Set to `true` to allow force pushes. Set to `false` or `null` to block force pushes. Default: `false`. For more information, see "[Enabling force pushes to a protected branch](https://docs.github.com/en/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)" in the GitHub Help documentation."
        #[serde(skip_serializing_if = "Option::is_none", default, deserialize_with = "crate::v1_1_4::support::deserialize_some")]
        pub allow_force_pushes: ::std::option::Option<::std::option::Option<bool>>,

        /// Allows deletion of the protected branch by anyone with write access to the repository. Set to `false` to prevent deletion of the protected branch. Default: `false`. For more information, see "[Enabling force pushes to a protected branch](https://docs.github.com/en/github/administering-a-repository/enabling-force-pushes-to-a-protected-branch)" in the GitHub Help documentation.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub allow_deletions: ::std::option::Option<bool>,

        /// Blocks creation of new branches which match the branch protection pattern. Set to `true` to prohibit new branch creation. Default: `false`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub block_creations: ::std::option::Option<bool>,

        /// Requires all conversations on code to be resolved before a pull request can be merged into a branch that matches this rule. Set to `false` to disable. Default: `false`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub required_conversation_resolution: ::std::option::Option<bool>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }

    /// Types for fields in [`Json`]
    pub mod json {
        /// Require status checks to pass before merging. Set to `null` to disable.
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct RequiredStatusChecks<'a> {
            /// Require branches to be up to date before merging.
            pub strict: bool,

            /// **Deprecated**: The list of status checks to require in order to merge into this branch. If any of these checks have recently been set by a particular GitHub App, they will be required to come from that app in future for the branch to merge. Use `checks` instead of `contexts` for more fine-grained control.
            pub contexts: ::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>,

            /// The list of status checks to require in order to merge into this branch.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub checks: ::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::repos_update_branch_protection::body::json::required_status_checks::Checks<'a>]>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }

        /// Require at least one approving review on a pull request, before merging. Set to `null` to disable.
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct RequiredPullRequestReviews<'a> {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub dismissal_restrictions: ::std::option::Option<crate::v1_1_4::request::repos_update_branch_protection::body::json::required_pull_request_reviews::DismissalRestrictions<'a>>,

            /// Set to `true` if you want to automatically dismiss approving reviews when someone pushes a new commit.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub dismiss_stale_reviews: ::std::option::Option<bool>,

            /// Blocks merging pull requests until [code owners](https://docs.github.com/articles/about-code-owners/) review them.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub require_code_owner_reviews: ::std::option::Option<bool>,

            /// Specify the number of reviewers required to approve pull requests. Use a number between 1 and 6 or 0 to not require reviewers.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub required_approving_review_count: ::std::option::Option<i64>,

            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub bypass_pull_request_allowances: ::std::option::Option<crate::v1_1_4::request::repos_update_branch_protection::body::json::required_pull_request_reviews::BypassPullRequestAllowances<'a>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }

        /// Restrict who can push to the protected branch. User, app, and team `restrictions` are only available for organization-owned repositories. Set to `null` to disable.
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Restrictions<'a> {
            /// The list of user `login`s with push access
            pub users: ::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>,

            /// The list of team `slug`s with push access
            pub teams: ::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>,

            /// The list of app `slug`s with push access
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub apps: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }

        /// Types for fields in [`RequiredStatusChecks`]
        pub mod required_status_checks {
            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct Checks<'a> {
                /// The name of the required check
                pub context: ::std::borrow::Cow<'a, str>,

                /// The ID of the GitHub App that must provide this check. Omit this field to automatically select the GitHub App that has recently provided this check, or any app if it was not set by a GitHub App. Pass -1 to explicitly allow any app to set the status.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub app_id: ::std::option::Option<i64>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }
        }

        /// Types for fields in [`RequiredPullRequestReviews`]
        pub mod required_pull_request_reviews {
            /// Specify which users and teams can dismiss pull request reviews. Pass an empty `dismissal_restrictions` object to disable. User and team `dismissal_restrictions` are only available for organization-owned repositories. Omit this parameter for personal repositories.
            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct DismissalRestrictions<'a> {
                /// The list of user `login`s with dismissal access
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub users: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

                /// The list of team `slug`s with dismissal access
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub teams: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }

            /// Allow specific users or teams to bypass pull request requirements.
            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct BypassPullRequestAllowances<'a> {
                /// The list of user `login`s allowed to bypass pull request requirements.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub users: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

                /// The list of team `slug`s allowed to bypass pull request requirements.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub teams: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }
        }
    }
}
