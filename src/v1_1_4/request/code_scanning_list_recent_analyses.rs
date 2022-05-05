
//! List code scanning analyses for a repository
//! 
//! Lists the details of all code scanning analyses for a repository,
//! starting with the most recent.
//! The response is paginated and you can use the `page` and `per_page` parameters
//! to list the analyses you're interested in.
//! By default 30 analyses are listed per page.
//! 
//! The `rules_count` field in the response give the number of rules
//! that were run in the analysis.
//! For very old analyses this data is not available,
//! and `0` is returned in this field.
//! 
//! You must use an access token with the `security_events` scope to use this endpoint with private repos,
//! the `public_repo` scope also grants permission to read security events on public repos only.
//! GitHub Apps must have the `security_events` read permission to use this endpoint.
//! 
//! **Deprecation notice**:
//! The `tool_name` field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/code-scanning#list-code-scanning-analyses-for-a-repository)


#[allow(clippy::too_many_arguments)]
fn url_string(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_ref: ::std::option::Option<&str>,
    q_sarif_id: ::std::option::Option<&str>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 49);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/code-scanning/analyses");
    let mut prefix = '?';
    if let Some(value) = &q_tool_name {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "tool_name", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_tool_guid {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "tool_guid", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_page {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_per_page {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "per_page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_ref {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "ref", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_sarif_id {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "sarif_id", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_ref: ::std::option::Option<&str>,
    q_sarif_id: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        q_tool_name,
        q_tool_guid,
        q_page,
        q_per_page,
        q_ref,
        q_sarif_id,
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
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_ref: ::std::option::Option<&str>,
    q_sarif_id: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        q_tool_name,
        q_tool_guid,
        q_page,
        q_per_page,
        q_ref,
        q_sarif_id,
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
    q_tool_name: ::std::option::Option<&str>,
    q_tool_guid: ::std::option::Option<::std::option::Option<&str>>,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_ref: ::std::option::Option<&str>,
    q_sarif_id: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        q_tool_name,
        q_tool_guid,
        q_page,
        q_per_page,
        q_ref,
        q_sarif_id,
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
