//! Update a provisioned organization membership
//! 
//! Replaces an existing provisioned user's information. You must provide all the information required for the user as if you were provisioning them for the first time. Any existing user information that you don't provide will be removed. If you want to only update a specific attribute, use the [Update an attribute for a SCIM user](https://docs.github.com/rest/reference/scim#update-an-attribute-for-a-scim-user) endpoint instead.
//! 
//! You must at least provide the required values for the user: `userName`, `name`, and `emails`.
//! 
//! **Warning:** Setting `active: false` removes the user from the organization, deletes the external identity, and deletes the associated `{scim_user_id}`.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/scim#set-scim-information-for-a-provisioned-user)

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
    p_scim_user_id: &str,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 48);
    url.push_str(trimmed);
    url.push_str("/scim/v2/organizations/");
    ::querylizer::Simple::extend(&mut url, &p_org, false, &::querylizer::encode_path)?;
    url.push_str("/Users/");
    ::querylizer::Simple::extend(&mut url, &p_scim_user_id, false, &::querylizer::encode_path)?;
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_org: &str,
    p_scim_user_id: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        p_scim_user_id,
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
    p_org: &str,
    p_scim_user_id: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        p_scim_user_id,
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
    p_org: &str,
    p_scim_user_id: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        p_scim_user_id,
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

/// Types for body parameter in [`super::scim_set_information_for_provisioned_user`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub schemas: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

        /// The name of the user, suitable for display to end-users
        /// 
        /// # Example
        /// 
        /// ```json
        /// "Jon Doe"
        /// ```
        #[serde(rename = "displayName", skip_serializing_if = "Option::is_none", default)]
        pub display_name: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        #[serde(rename = "externalId", skip_serializing_if = "Option::is_none", default)]
        pub external_id: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub groups: ::std::option::Option<::std::borrow::Cow<'a, [::std::borrow::Cow<'a, str>]>>,

        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub active: ::std::option::Option<bool>,

        /// Configured by the admin. Could be an email, login, or username
        /// 
        /// # Example
        /// 
        /// ```json
        /// "someone@example.com"
        /// ```
        #[serde(rename = "userName")]
        pub user_name: ::std::borrow::Cow<'a, str>,

        pub name: crate::v1_1_4::request::scim_set_information_for_provisioned_user::body::json::Name<'a>,

        /// user emails
        /// 
        /// # Example
        /// 
        /// ```json
        /// [
        ///   {
        ///     "primary": true,
        ///     "value": "someone@example.com"
        ///   },
        ///   {
        ///     "primary": false,
        ///     "value": "another@example.com"
        ///   }
        /// ]
        /// ```
        pub emails: ::std::borrow::Cow<'a, [crate::v1_1_4::request::scim_set_information_for_provisioned_user::body::json::Emails<'a>]>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }

    /// Types for fields in [`Json`]
    pub mod json {
        /// # Example
        /// 
        /// ```json
        /// {
        ///   "familyName": "User",
        ///   "givenName": "Jane"
        /// }
        /// ```
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Name<'a> {
            #[serde(rename = "givenName")]
            pub given_name: ::std::borrow::Cow<'a, str>,

            #[serde(rename = "familyName")]
            pub family_name: ::std::borrow::Cow<'a, str>,

            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub formatted: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }

        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Emails<'a> {
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub r#type: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            pub value: ::std::borrow::Cow<'a, str>,

            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub primary: ::std::option::Option<bool>,

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
