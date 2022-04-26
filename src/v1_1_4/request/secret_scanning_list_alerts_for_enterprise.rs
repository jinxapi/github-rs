
//! List secret scanning alerts for an enterprise
//! 
//! Lists secret scanning alerts for eligible repositories in an enterprise, from newest to oldest.
//! To use this endpoint, you must be a member of the enterprise, and you must use an access token with the `repo` scope or `security_events` scope. Alerts are only returned for organizations in the enterprise for which you are an organization owner or a [security manager](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).
//! 
//! [API method documentation](https://docs.github.com/rest/reference/secret-scanning#list-secret-scanning-alerts-for-an-enterprise)


#[allow(clippy::too_many_arguments)]
fn url_string(
    base_url: &str,
    p_enterprise: &str,
    q_state: ::std::option::Option<&str>,
    q_secret_type: ::std::option::Option<&str>,
    q_resolution: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 53);
    url.push_str(trimmed);
    url.push_str("/enterprises/");
    ::querylizer::Simple::extend(&mut url, &p_enterprise, false, &::querylizer::encode_path)?;
    url.push_str("/secret-scanning/alerts");
    let mut prefix = ::std::iter::once('?').fuse();
    if let Some(value) = &q_state {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "state", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_secret_type {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "secret_type", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_resolution {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "resolution", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_per_page {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "per_page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_before {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "before", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_after {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "after", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    p_enterprise: &str,
    q_state: ::std::option::Option<&str>,
    q_secret_type: ::std::option::Option<&str>,
    q_resolution: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_enterprise,
        q_state,
        q_secret_type,
        q_resolution,
        q_per_page,
        q_before,
        q_after,
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
    p_enterprise: &str,
    q_state: ::std::option::Option<&str>,
    q_secret_type: ::std::option::Option<&str>,
    q_resolution: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_enterprise,
        q_state,
        q_secret_type,
        q_resolution,
        q_per_page,
        q_before,
        q_after,
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
    p_enterprise: &str,
    q_state: ::std::option::Option<&str>,
    q_secret_type: ::std::option::Option<&str>,
    q_resolution: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_before: ::std::option::Option<&str>,
    q_after: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_enterprise,
        q_state,
        q_secret_type,
        q_resolution,
        q_per_page,
        q_before,
        q_after,
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
