//! Get top referral paths
//! 
//! Get the top 10 popular contents over the last 14 days.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#get-top-referral-paths)


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
    let mut url = String::with_capacity(trimmed.len() + 48);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/traffic/popular/paths");
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
