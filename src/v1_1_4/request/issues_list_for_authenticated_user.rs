//! List user account issues assigned to the authenticated user
//! 
//! List issues across owned and member repositories assigned to the authenticated user.
//! 
//! **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
//! reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
//! the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
//! request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/issues#list-user-account-issues-assigned-to-the-authenticated-user)


#[allow(clippy::too_many_arguments)]
fn url_string(
    base_url: &str,
    q_filter: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_labels: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
    q_direction: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 28);
    url.push_str(trimmed);
    url.push_str("/user/issues");
    let mut prefix = '?';
    if let Some(value) = &q_filter {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "filter", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_state {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "state", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_labels {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "labels", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_sort {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "sort", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_direction {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "direction", value, false, &::querylizer::encode_query)?;
    }
    if let Some(value) = &q_since {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "since", value, false, &::querylizer::encode_query)?;
    }
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
#[allow(clippy::too_many_arguments)]
pub fn http_builder(
    base_url: &str,
    q_filter: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_labels: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
    q_direction: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        q_filter,
        q_state,
        q_labels,
        q_sort,
        q_direction,
        q_since,
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
    q_filter: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_labels: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
    q_direction: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        q_filter,
        q_state,
        q_labels,
        q_sort,
        q_direction,
        q_since,
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
    q_filter: ::std::option::Option<&str>,
    q_state: ::std::option::Option<&str>,
    q_labels: ::std::option::Option<&str>,
    q_sort: ::std::option::Option<&str>,
    q_direction: ::std::option::Option<&str>,
    q_since: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        q_filter,
        q_state,
        q_labels,
        q_sort,
        q_direction,
        q_since,
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
