
//! List provisioned SCIM groups for an enterprise
//! 
//! **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/enterprise-admin#list-provisioned-scim-groups-for-an-enterprise)


fn url_string(
    base_url: &str,
    p_enterprise: &str,
    q_start_index: ::std::option::Option<i64>,
    q_count: ::std::option::Option<i64>,
    q_filter: ::std::option::Option<&str>,
    q_excluded_attributes: ::std::option::Option<&str>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 45);
    url.push_str(trimmed);
    url.push_str("/scim/v2/enterprises/");
    ::querylizer::Simple::extend(&mut url, &p_enterprise, false, &::querylizer::encode_path)?;
    url.push_str("/Groups");
    let mut prefix = ::std::iter::once('?').fuse();
    if let Some(value) = &q_start_index {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "startIndex", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_count {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "count", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_filter {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "filter", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_excluded_attributes {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "excludedAttributes", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    p_enterprise: &str,
    q_start_index: ::std::option::Option<i64>,
    q_count: ::std::option::Option<i64>,
    q_filter: ::std::option::Option<&str>,
    q_excluded_attributes: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_enterprise,
        q_start_index,
        q_count,
        q_filter,
        q_excluded_attributes,
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
    q_start_index: ::std::option::Option<i64>,
    q_count: ::std::option::Option<i64>,
    q_filter: ::std::option::Option<&str>,
    q_excluded_attributes: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_enterprise,
        q_start_index,
        q_count,
        q_filter,
        q_excluded_attributes,
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
    q_start_index: ::std::option::Option<i64>,
    q_count: ::std::option::Option<i64>,
    q_filter: ::std::option::Option<&str>,
    q_excluded_attributes: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_enterprise,
        q_start_index,
        q_count,
        q_filter,
        q_excluded_attributes,
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
