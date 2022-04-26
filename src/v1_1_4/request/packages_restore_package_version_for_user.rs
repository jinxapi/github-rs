
//! Restore package version for a user
//! 
//! Restores a specific package version for a user.
//! 
//! You can restore a deleted package under the following conditions:
//!   - The package was deleted within the last 30 days.
//!   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
//! 
//! To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:write` scopes. In addition:
//! - If `package_type` is not `container`, your token must also include the `repo` scope.
//! - If `package_type` is `container`, you must also have admin permissions to the container that you want to restore.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/packages#restore-a-package-version-for-a-user)


fn url_string(
    base_url: &str,
    p_package_type: &str,
    p_package_name: &str,
    p_username: &str,
    p_package_version_id: i64,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 56);
    url.push_str(trimmed);
    url.push_str("/users/");
    ::querylizer::Simple::extend(&mut url, &p_username, false, &::querylizer::encode_path)?;
    url.push_str("/packages/");
    ::querylizer::Simple::extend(&mut url, &p_package_type, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_package_name, false, &::querylizer::encode_path)?;
    url.push_str("/versions/");
    ::querylizer::Simple::extend(&mut url, &p_package_version_id, false, &::querylizer::encode_path)?;
    url.push_str("/restore");
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_package_type: &str,
    p_package_name: &str,
    p_username: &str,
    p_package_version_id: i64,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_package_type,
        p_package_name,
        p_username,
        p_package_version_id,
    )?;
    let mut builder = ::http::request::Request::post(url);
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
    p_package_type: &str,
    p_package_name: &str,
    p_username: &str,
    p_package_version_id: i64,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_package_type,
        p_package_name,
        p_username,
        p_package_version_id,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::Request::new(::reqwest::Method::POST, reqwest_url);
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
    p_package_type: &str,
    p_package_name: &str,
    p_username: &str,
    p_package_version_id: i64,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_package_type,
        p_package_name,
        p_username,
        p_package_version_id,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::blocking::Request::new(::reqwest::Method::POST, reqwest_url);
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
