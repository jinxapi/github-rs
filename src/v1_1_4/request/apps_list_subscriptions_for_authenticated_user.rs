//! List subscriptions for the authenticated user
//! 
//! Lists the active subscriptions for the authenticated user. You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint. . OAuth Apps must authenticate using an [OAuth token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/).
//! 
//! [API method documentation](https://docs.github.com/rest/reference/apps#list-subscriptions-for-the-authenticated-user)


fn url_string(
    base_url: &str,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 43);
    url.push_str(trimmed);
    url.push_str("/user/marketplace_purchases");
    let mut prefix = '?';
    if let Some(value) = &q_per_page {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "per_page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_page {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "page", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        q_per_page,
        q_page,
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
pub fn reqwest_builder(
    base_url: &str,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        q_per_page,
        q_page,
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
pub fn reqwest_blocking_builder(
    base_url: &str,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        q_per_page,
        q_page,
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
