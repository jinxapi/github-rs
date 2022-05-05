
//! List code scanning alerts for an organization
//! 
//! Lists all code scanning alerts for the default branch (usually `main`
//! or `master`) for all eligible repositories in an organization.
//! To use this endpoint, you must be an administrator or security manager for the organization, and you must use an access token with the `repo` scope or `security_events` scope.
//! 
//! GitHub Apps must have the `security_events` read permission to use this endpoint.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/code-scanning#list-code-scanning-alerts-by-organization)


#[allow(clippy::too_many_arguments)]
fn url_string(
    base_url: &str,
    p_org: &str,
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_direction: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 44);
    url.push_str(trimmed);
    url.push_str("/orgs/");
    ::querylizer::Simple::extend(&mut url, &p_org, false, &::querylizer::encode_path)?;
    url.push_str("/code-scanning/alerts");
    let mut prefix = '?';
    if let Some(value) = &q_tool_name {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "tool_name", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_tool_guid {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "tool_guid", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_before {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "before", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_after {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "after", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_page {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_per_page {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "per_page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_direction {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "direction", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_state {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "state", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_sort {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "sort", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    p_org: &str,
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_direction: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        q_tool_name,
        q_tool_guid,
        q_before,
        q_after,
        q_page,
        q_per_page,
        q_direction,
        q_state,
        q_sort,
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
    p_org: &str,
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_direction: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        q_tool_name,
        q_tool_guid,
        q_before,
        q_after,
        q_page,
        q_per_page,
        q_direction,
        q_state,
        q_sort,
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
    p_org: &str,
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_direction: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        q_tool_name,
        q_tool_guid,
        q_before,
        q_after,
        q_page,
        q_per_page,
        q_direction,
        q_state,
        q_sort,
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
