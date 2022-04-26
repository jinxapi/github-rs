
//! Create a check run
//! 
//! **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
//! 
//! Creates a new check run for a specific commit in a repository. Your GitHub App must have the `checks:write` permission to create check runs.
//! 
//! In a check suite, GitHub limits the number of check runs with the same name to 1000. Once these check runs exceed 1000, GitHub will start to automatically delete older check runs.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/checks#create-a-check-run)

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
    let mut url = String::with_capacity(trimmed.len() + 37);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/check-runs");
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
impl<'a> TryFrom<&crate::v1_1_4::request::checks_create::body::Json<'a>> for Content<::hyper::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::checks_create::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest")]
impl<'a> TryFrom<&crate::v1_1_4::request::checks_create::body::Json<'a>> for Content<::reqwest::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::checks_create::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

#[cfg(feature = "reqwest-blocking")]
impl<'a> TryFrom<&crate::v1_1_4::request::checks_create::body::Json<'a>> for Content<::reqwest::blocking::Body> {
    type Error = crate::v1_1_4::ApiError;

    fn try_from(value: &crate::v1_1_4::request::checks_create::body::Json<'a>) -> Result<Self, Self::Error> {
        Ok(
            Self::new(::serde_json::to_vec(value)?.into())
            .with_content_type(&b"application/json"[..])
        )
    }
}

/// Types for body parameter in [`super::checks_create`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// The name of the check. For example, "code-coverage".
        pub name: ::std::borrow::Cow<'a, str>,

        /// The SHA of the commit.
        pub head_sha: ::std::borrow::Cow<'a, str>,

        /// The URL of the integrator's site that has the full details of the check. If the integrator does not provide this, then the homepage of the GitHub app is used.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub details_url: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// A reference for the run on the integrator's system.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub external_id: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The current status. Can be one of `queued`, `in_progress`, or `completed`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub status: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub started_at: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// **Required if you provide `completed_at` or a `status` of `completed`**. The final conclusion of the check. Can be one of `action_required`, `cancelled`, `failure`, `neutral`, `success`, `skipped`, `stale`, or `timed_out`. When the conclusion is `action_required`, additional details should be provided on the site specified by `details_url`.  
        /// **Note:** Providing `conclusion` will automatically set the `status` parameter to `completed`. You cannot change a check run conclusion to `stale`, only GitHub can set this.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub conclusion: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        /// The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub completed_at: ::std::option::Option<::std::borrow::Cow<'a, str>>,

        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub output: ::std::option::Option<crate::v1_1_4::request::checks_create::body::json::Output<'a>>,

        /// Displays a button on GitHub that can be clicked to alert your app to do additional tasks. For example, a code linting app can display a button that automatically fixes detected errors. The button created in this object is displayed after the check run completes. When a user clicks the button, GitHub sends the [`check_run.requested_action` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) to your app. Each action includes a `label`, `identifier` and `description`. A maximum of three actions are accepted. See the [`actions` object](https://docs.github.com/rest/reference/checks#actions-object) description. To learn more about check runs and requested actions, see "[Check runs and requested actions](https://docs.github.com/rest/reference/checks#check-runs-and-requested-actions)."
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub actions: ::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::checks_create::body::json::Actions<'a>]>>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }

    /// Types for fields in [`Json`]
    pub mod json {
        /// Check runs can accept a variety of data in the `output` object, including a `title` and `summary` and can optionally provide descriptive details about the run. See the [`output` object](https://docs.github.com/rest/reference/checks#output-object) description.
        #[allow(non_snake_case)]
        #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct Output<'a> {
            /// The title of the check run.
            pub title: ::std::borrow::Cow<'a, str>,

            /// The summary of the check run. This parameter supports Markdown.
            pub summary: ::std::borrow::Cow<'a, str>,

            /// The details of the check run. This parameter supports Markdown.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub text: ::std::option::Option<::std::borrow::Cow<'a, str>>,

            /// Adds information from your analysis to specific lines of code. Annotations are visible on GitHub in the **Checks** and **Files changed** tab of the pull request. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/reference/checks#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. For details about how you can view annotations on GitHub, see "[About status checks](https://help.github.com/articles/about-status-checks#checks)". See the [`annotations` object](https://docs.github.com/rest/reference/checks#annotations-object) description for details about how to use this parameter.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub annotations: ::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::checks_create::body::json::output::Annotations<'a>]>>,

            /// Adds images to the output displayed in the GitHub pull request UI. See the [`images` object](https://docs.github.com/rest/reference/checks#images-object) description for details.
            #[serde(skip_serializing_if = "Option::is_none", default)]
            pub images: ::std::option::Option<::std::borrow::Cow<'a, [crate::v1_1_4::request::checks_create::body::json::output::Images<'a>]>>,

            #[serde(flatten)]
            pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
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
    }
}
