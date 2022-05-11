//! Update an organization
//! 
//! **Parameter Deprecation Notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).
//! 
//! Enables an authenticated organization owner with the `admin:org` scope to update the organization's profile and member privileges.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/orgs/#update-an-organization)

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
    let mut url = String::with_capacity(trimmed.len() + 23);
    url.push_str(trimmed);
    url.push_str("/orgs/");
    ::querylizer::Simple::extend(&mut url, &p_org, false, &::querylizer::encode_path)?;
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
    p_org: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
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
    p_org: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
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

/// Types for body parameter in [`super::orgs_update`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// Billing email address. This address is not publicized.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub billing_email: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The company name.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub company: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The publicly visible email address.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub email: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The Twitter username of the company.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub twitter_username: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The location.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub location: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The shorthand name of the company.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub name: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The description of the company.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub description: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Toggles whether an organization can use organization projects.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_organization_projects: ::std::option::Option<bool>,

        /// Toggles whether repositories that belong to the organization can use repository projects.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub has_repository_projects: ::std::option::Option<bool>,

        /// Default permission level members have for organization repositories:  
        /// \* `read` - can pull, but not push to or administer this repository.  
        /// \* `write` - can pull and push, but not administer this repository.  
        /// \* `admin` - can pull, push, and administer this repository.  
        /// \* `none` - no permissions granted by default.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub default_repository_permission: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Toggles the ability of non-admin organization members to create repositories. Can be one of:  
        /// \* `true` - all organization members can create repositories.  
        /// \* `false` - only organization owners can create repositories.  
        /// Default: `true`  
        /// **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details. **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_create_repositories: ::std::option::Option<bool>,

        /// Toggles whether organization members can create internal repositories, which are visible to all enterprise members. You can only allow members to create internal repositories if your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. Can be one of:  
        /// \* `true` - all organization members can create internal repositories.  
        /// \* `false` - only organization owners can create internal repositories.  
        /// Default: `true`. For more information, see "[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)" in the GitHub Help documentation.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_create_internal_repositories: ::std::option::Option<bool>,

        /// Toggles whether organization members can create private repositories, which are visible to organization members with permission. Can be one of:  
        /// \* `true` - all organization members can create private repositories.  
        /// \* `false` - only organization owners can create private repositories.  
        /// Default: `true`. For more information, see "[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)" in the GitHub Help documentation.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_create_private_repositories: ::std::option::Option<bool>,

        /// Toggles whether organization members can create public repositories, which are visible to anyone. Can be one of:  
        /// \* `true` - all organization members can create public repositories.  
        /// \* `false` - only organization owners can create public repositories.  
        /// Default: `true`. For more information, see "[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)" in the GitHub Help documentation.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_create_public_repositories: ::std::option::Option<bool>,

        /// Specifies which types of repositories non-admin organization members can create. Can be one of:  
        /// \* `all` - all organization members can create public and private repositories.  
        /// \* `private` - members can create private repositories. This option is only available to repositories that are part of an organization on GitHub Enterprise Cloud.  
        /// \* `none` - only admin members can create repositories.  
        /// **Note:** This parameter is deprecated and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_allowed_repository_creation_type: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// Toggles whether organization members can create GitHub Pages sites. Can be one of:  
        /// \* `true` - all organization members can create GitHub Pages sites.  
        /// \* `false` - no organization members can create GitHub Pages sites. Existing published sites will not be impacted.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_create_pages: ::std::option::Option<bool>,

        /// Toggles whether organization members can create public GitHub Pages sites. Can be one of:  
        /// \* `true` - all organization members can create public GitHub Pages sites.  
        /// \* `false` - no organization members can create public GitHub Pages sites. Existing published sites will not be impacted.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_create_public_pages: ::std::option::Option<bool>,

        /// Toggles whether organization members can create private GitHub Pages sites. Can be one of:  
        /// \* `true` - all organization members can create private GitHub Pages sites.  
        /// \* `false` - no organization members can create private GitHub Pages sites. Existing published sites will not be impacted.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_create_private_pages: ::std::option::Option<bool>,

        /// Toggles whether organization members can fork private organization repositories. Can be one of:  
        /// \* `true` - all organization members can fork private repositories within the organization.  
        /// \* `false` - no organization members can fork private repositories within the organization.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub members_can_fork_private_repositories: ::std::option::Option<bool>,

        /// # Example
        /// 
        /// ```json
        /// "\"http://github.blog\""
        /// ```
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub blog: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
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
