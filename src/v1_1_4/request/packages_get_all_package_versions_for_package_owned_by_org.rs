
//! Get all package versions for a package owned by an organization
//! 
//! Returns all package versions for a package owned by an organization.
//! 
//! To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
//! If `package_type` is not `container`, your token must also include the `repo` scope.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-an-organization)


fn url_string(
    base_url: &str,
    p_package_type: &str,
    p_package_name: &str,
    p_org: &str,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_state: ::std::option::Option<&str>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 45);
    url.push_str(trimmed);
    url.push_str("/orgs/");
    ::querylizer::Simple::extend(&mut url, &p_org, false, &::querylizer::encode_path)?;
    url.push_str("/packages/");
    ::querylizer::Simple::extend(&mut url, &p_package_type, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_package_name, false, &::querylizer::encode_path)?;
    url.push_str("/versions");
    let mut prefix = ::std::iter::once('?').fuse();
    if let Some(value) = &q_page {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_per_page {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "per_page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_state {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "state", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    p_package_type: &str,
    p_package_name: &str,
    p_org: &str,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_state: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_package_type,
        p_package_name,
        p_org,
        q_page,
        q_per_page,
        q_state,
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
    p_package_type: &str,
    p_package_name: &str,
    p_org: &str,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_state: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_package_type,
        p_package_name,
        p_org,
        q_page,
        q_per_page,
        q_state,
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
    p_package_type: &str,
    p_package_name: &str,
    p_org: &str,
    q_page: ::std::option::Option<i64>,
    q_per_page: ::std::option::Option<i64>,
    q_state: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_package_type,
        p_package_name,
        p_org,
        q_page,
        q_per_page,
        q_state,
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
