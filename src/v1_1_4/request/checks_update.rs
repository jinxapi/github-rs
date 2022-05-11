//! Update a check run
//! 
//! **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
//! 
//! Updates a check run for a specific commit in a repository. Your GitHub App must have the `checks:write` permission to edit check runs.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/checks#update-a-check-run)

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
    p_check_run_id: i64,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 39);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/check-runs/");
    ::querylizer::Simple::extend(&mut url, &p_check_run_id, false, &::querylizer::encode_path)?;
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_check_run_id: i64,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_check_run_id,
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
    p_check_run_id: i64,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_check_run_id,
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
    p_check_run_id: i64,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_check_run_id,
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

/// Types for body parameter in [`super::checks_update`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The name of the check. For example, "code-coverage".
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub name: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The URL of the integrator's site that has the full details of the check.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub details_url: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// A reference for the run on the integrator's system.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub external_id: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub started_at: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The current status. Can be one of `queued`, `in_progress`, or `completed`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub status: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`.  
        /// **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub conclusion: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub completed_at: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub output: ::std::option::Option<crate::v1_1_4::request::checks_update::body::json::Output<'a>>,

        /// Possible further actions the integrator can perform, which a user may trigger. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see "[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions)."
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub actions: ::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::checks_update::body::json::Actions<'a>]>>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }

    /// Types for fields in [`Json`]
    pub mod json {
        /// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object-1) description.
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Output<'a> {
            /// **Required**.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub title: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            /// Can contain Markdown.
            pub summary: ::std::borrow::Cow<'a, str>,

            /// Can contain Markdown.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub text: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            /// Adds information from your analysis to specific lines of code. Annotations are visible in GitHub's pull request UI. Annotations are visible in GitHub's pull request UI. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/reference/checks#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. For details about annotations in the UI, see "[About status checks](https://docs.github.com/articles/about-status-checks#checks)". See the [`annotations` object](https://docs.github.com/rest/reference/checks#annotations-object-1) description for details.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub annotations: ::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::checks_update::body::json::output::Annotations<'a>]>>,

            /// Adds images to the output displayed in the GitHub pull request UI. See the [`images` object](https://docs.github.com/rest/reference/checks#annotations-object-1) description for details.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub images: ::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::checks_update::body::json::output::Images<'a>]>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
        }

        /// Types for fields in [`Output`]
        pub mod output {
            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct Annotations<'a> {
                /// The path of the file to add an annotation to. For example, `assets/css/main.css`.
                pub path: ::std::borrow::Cow<'a, str>,

                /// The start line of the annotation.
                pub start_line: i64,

                /// The end line of the annotation.
                pub end_line: i64,

                /// The start column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub start_column: ::std::option::Option<i64>,

                /// The end column of the annotation. Annotations only support `start_column` and `end_column` on the same line. Omit this parameter if `start_line` and `end_line` have different values.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub end_column: ::std::option::Option<i64>,

                /// The level of the annotation. Can be one of `notice`, `warning`, or `failure`.
                pub annotation_level: ::std::borrow::Cow<'a, str>,

                /// A short description of the feedback for these lines of code. The maximum size is 64 KB.
                pub message: ::std::borrow::Cow<'a, str>,

                /// The title that represents the annotation. The maximum size is 255 characters.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub title: ::std::option::Option<::std::borrow::Cow<'a, str>>,

                /// Details about this annotation. The maximum size is 64 KB.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub raw_details: ::std::option::Option<::std::borrow::Cow<'a, str>>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }

            #[allow(non_snake_case)]
            #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
            pub struct Images<'a> {
                /// The alternative text for the image.
                pub alt: ::std::borrow::Cow<'a, str>,

                /// The full URL of the image.
                pub image_url: ::std::borrow::Cow<'a, str>,

                /// A short image description.
                #[serde(skip_serializing_if = "Option::is_none", default)]
                pub caption: ::std::option::Option<::std::borrow::Cow<'a, str>>,

                #[serde(flatten)]
                pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
            }
        }

        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Actions<'a> {
            /// The text to be displayed on a button in the web UI. The maximum size is 20 characters.
            pub label: ::std::borrow::Cow<'a, str>,

            /// A short explanation of what this action would do. The maximum size is 40 characters.
            pub description: ::std::borrow::Cow<'a, str>,

            /// A reference for the action on the integrator's system. The maximum size is 20 characters.
            pub identifier: ::std::borrow::Cow<'a, str>,

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
