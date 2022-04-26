
//! List workflow runs
//! 
//! List all workflow runs for a workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
//! 
//! Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/actions#list-workflow-runs)


#[allow(clippy::too_many_arguments)]
fn url_string(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_workflow_id: &::serde_json::value::Value,
    q_actor: ::std::option::Option<&str>,
    q_branch: ::std::option::Option<&str>,
    q_event: ::std::option::Option<&str>,
    q_status: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    q_created: ::std::option::Option<&str>,
    q_exclude_pull_requests: ::std::option::Option<bool>,
    q_check_suite_id: ::std::option::Option<i64>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 51);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/actions/workflows/");
    ::querylizer::Simple::extend(&mut url, p_workflow_id, false, &::querylizer::encode_path)?;
    url.push_str("/runs");
    let mut prefix = ::std::iter::once('?').fuse();
    if let Some(value) = &q_actor {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "actor", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_branch {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "branch", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_event {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "event", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_status {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "status", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_per_page {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "per_page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_page {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_created {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "created", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_exclude_pull_requests {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "exclude_pull_requests", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_check_suite_id {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "check_suite_id", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_workflow_id: &::serde_json::value::Value,
    q_actor: ::std::option::Option<&str>,
    q_branch: ::std::option::Option<&str>,
    q_event: ::std::option::Option<&str>,
    q_status: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    q_created: ::std::option::Option<&str>,
    q_exclude_pull_requests: ::std::option::Option<bool>,
    q_check_suite_id: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_workflow_id,
        q_actor,
        q_branch,
        q_event,
        q_status,
        q_per_page,
        q_page,
        q_created,
        q_exclude_pull_requests,
        q_check_suite_id,
    )?;
    let mut builder = ::http::request::Request::get(url);
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
#[inline]
pub fn hyper_request(
    builder: ::http::request::Builder,
) -> Result<::http::request::Request<::hyper::Body>, crate::v1_1_4::ApiError> {
    Ok(builder.body(::hyper::Body::empty())?)
}

#[cfg(feature = "reqwest")]
#[allow(clippy::too_many_arguments)]
pub fn reqwest_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_workflow_id: &::serde_json::value::Value,
    q_actor: ::std::option::Option<&str>,
    q_branch: ::std::option::Option<&str>,
    q_event: ::std::option::Option<&str>,
    q_status: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    q_created: ::std::option::Option<&str>,
    q_exclude_pull_requests: ::std::option::Option<bool>,
    q_check_suite_id: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_workflow_id,
        q_actor,
        q_branch,
        q_event,
        q_status,
        q_per_page,
        q_page,
        q_created,
        q_exclude_pull_requests,
        q_check_suite_id,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::Request::new(::reqwest::Method::GET, reqwest_url);
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
#[inline(always)]
pub fn reqwest_request(
    builder: ::reqwest::Request,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError>
{
    Ok(builder)
}

#[cfg(feature = "reqwest-blocking")]
#[allow(clippy::too_many_arguments)]
pub fn reqwest_blocking_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_workflow_id: &::serde_json::value::Value,
    q_actor: ::std::option::Option<&str>,
    q_branch: ::std::option::Option<&str>,
    q_event: ::std::option::Option<&str>,
    q_status: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    q_created: ::std::option::Option<&str>,
    q_exclude_pull_requests: ::std::option::Option<bool>,
    q_check_suite_id: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_workflow_id,
        q_actor,
        q_branch,
        q_event,
        q_status,
        q_per_page,
        q_page,
        q_created,
        q_exclude_pull_requests,
        q_check_suite_id,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::blocking::Request::new(::reqwest::Method::GET, reqwest_url);
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
#[inline(always)]
pub fn reqwest_blocking_request(
    builder: ::reqwest::blocking::Request,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError>
{
    Ok(builder)
}
