
//! List commits
//! 
//! **Signature verification object**
//! 
//! The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
//! 
//! | Name | Type | Description |
//! | ---- | ---- | ----------- |
//! | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
//! | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
//! | `signature` | `string` | The signature that was extracted from the commit. |
//! | `payload` | `string` | The value that was signed. |
//! 
//! These are the possible values for `reason` in the `verification` object:
//! 
//! | Value | Description |
//! | ----- | ----------- |
//! | `expired_key` | The key that made the signature is expired. |
//! | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
//! | `gpgverify_error` | There was an error communicating with the signature verification service. |
//! | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
//! | `unsigned` | The object does not include a signature. |
//! | `unknown_signature_type` | A non-PGP signature was found in the commit. |
//! | `no_user` | No user was associated with the `committer` email address in the commit. |
//! | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
//! | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
//! | `unknown_key` | The key that made the signature has not been registered with any user's account. |
//! | `malformed_signature` | There was an error parsing the signature. |
//! | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
//! | `valid` | None of the above errors applied, so the signature is considered to be verified. |
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#list-commits)


#[allow(clippy::too_many_arguments)]
fn url_string(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    q_sha: ::std::option::Option<&str>,
    q_path: ::std::option::Option<&str>,
    q_author: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_until: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 34);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/commits");
    let mut prefix = ::std::iter::once('?').fuse();
    if let Some(value) = &q_sha {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "sha", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_path {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "path", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_author {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "author", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_since {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "since", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_until {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "until", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_per_page {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "per_page", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_page {
        url.push(prefix.next().unwrap_or('&'));
        ::querylizer::Form::extend(&mut url, "page", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    q_sha: ::std::option::Option<&str>,
    q_path: ::std::option::Option<&str>,
    q_author: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_until: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        q_sha,
        q_path,
        q_author,
        q_since,
        q_until,
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
#[allow(clippy::too_many_arguments)]
pub fn reqwest_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    q_sha: ::std::option::Option<&str>,
    q_path: ::std::option::Option<&str>,
    q_author: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_until: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        q_sha,
        q_path,
        q_author,
        q_since,
        q_until,
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
#[allow(clippy::too_many_arguments)]
pub fn reqwest_blocking_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    q_sha: ::std::option::Option<&str>,
    q_path: ::std::option::Option<&str>,
    q_author: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_until: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        q_sha,
        q_path,
        q_author,
        q_since,
        q_until,
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