
//! Get feeds
//! 
//! GitHub provides several timeline resources in [Atom](http://en.wikipedia.org/wiki/Atom_(standard)) format. The Feeds API lists all the feeds available to the authenticated user:
//! 
//! *   **Timeline**: The GitHub global public timeline
//! *   **User**: The public timeline for any user, using [URI template](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia)
//! *   **Current user public**: The public timeline for the authenticated user
//! *   **Current user**: The private timeline for the authenticated user
//! *   **Current user actor**: The private timeline for activity created by the authenticated user
//! *   **Current user organizations**: The private timeline for the organizations the authenticated user is a member of.
//! *   **Security advisories**: A collection of public announcements that provide information about security-related vulnerabilities in software on GitHub.
//! 
//! **Note**: Private feeds are only returned when [authenticating via Basic Auth](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) since current feed URIs use the older, non revocable auth tokens.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/activity#get-feeds)


#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/feeds");
    let url = if base_url.is_empty() {
        ::http::uri::Uri::from_static(default_url)
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 6);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        url.try_into().map_err(::http::Error::from)?
    };
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
pub fn reqwest_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/feeds");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 6);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
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
pub fn reqwest_blocking_builder(
    base_url: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let default_url = concat!("https://api.github.com", "/feeds");
    let reqwest_url = if base_url.is_empty() {
        ::reqwest::Url::parse(default_url)?
    } else {
        let trimmed = base_url.trim_end_matches('/');
        let mut url = String::with_capacity(trimmed.len() + 6);
        url.push_str(trimmed);
        url.push_str(&default_url[22..]);
        ::reqwest::Url::parse(&url)?
    };
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
