//! List reactions for a team discussion
//! 
//! List the reactions to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `read:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
//! 
//! **Note:** You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/reactions#list-reactions-for-a-team-discussion)


fn url_string(
    base_url: &str,
    p_org: &str,
    p_team_slug: &str,
    p_discussion_number: i64,
    q_content: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 55);
    url.push_str(trimmed);
    url.push_str("/orgs/");
    ::querylizer::Simple::extend(&mut url, &p_org, false, &::querylizer::encode_path)?;
    url.push_str("/teams/");
    ::querylizer::Simple::extend(&mut url, &p_team_slug, false, &::querylizer::encode_path)?;
    url.push_str("/discussions/");
    ::querylizer::Simple::extend(&mut url, &p_discussion_number, false, &::querylizer::encode_path)?;
    url.push_str("/reactions");
    let mut prefix = '?';
    if let Some(value) = &q_content {
        url.push(::std::mem::replace(&mut prefix, '&'));
        ::querylizer::Form::extend(&mut url, "content", value, false, &::querylizer::encode_query)?;
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
    p_org: &str,
    p_team_slug: &str,
    p_discussion_number: i64,
    q_content: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        p_team_slug,
        p_discussion_number,
        q_content,
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
    p_org: &str,
    p_team_slug: &str,
    p_discussion_number: i64,
    q_content: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        p_team_slug,
        p_discussion_number,
        q_content,
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
    p_org: &str,
    p_team_slug: &str,
    p_discussion_number: i64,
    q_content: ::std::option::Option<&str>,
    q_per_page: ::std::option::Option<i64>,
    q_page: ::std::option::Option<i64>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_org,
        p_team_slug,
        p_discussion_number,
        q_content,
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
