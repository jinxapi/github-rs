
//! Revoke an installation access token
//! 
//! Revokes the installation token you're using to authenticate as an installation and access this endpoint.
//! 
//! Once an installation token is revoked, the token is invalidated and cannot be used. Other endpoints that require the revoked installation token must have a new installation token to work. You can create a new token using the "[Create an installation access token for an app](https://docs.github.com/rest/reference/apps#create-an-installation-access-token-for-an-app)" endpoint.
//! 
//! You must use an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/apps#revoke-an-installation-access-token)


#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/installation/token");
    let url = if base_url.is_empty() {
        ::http::uri::Uri::from_static(default_url)
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 19);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        url.try_into().map_err(::http::Error::from)?
    };
    let mut builder = ::http::request::Request::delete(url);
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
pub fn reqwest_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/installation/token");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 19);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
    let mut request = ::reqwest::Request::new(::reqwest::Method::DELETE, reqwest_url);
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
pub fn reqwest_blocking_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/installation/token");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 19);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
    let mut request = ::reqwest::blocking::Request::new(::reqwest::Method::DELETE, reqwest_url);
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
