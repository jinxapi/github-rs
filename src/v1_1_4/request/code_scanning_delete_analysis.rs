
//! Delete a code scanning analysis from a repository
//! 
//! Deletes a specified code scanning analysis from a repository. For
//! private repositories, you must use an access token with the `repo` scope. For public repositories,
//! you must use an access token with `public_repo` scope.
//! GitHub Apps must have the `security_events` write permission to use this endpoint.
//! 
//! You can delete one analysis at a time.
//! To delete a series of analyses, start with the most recent analysis and work backwards.
//! Conceptually, the process is similar to the undo function in a text editor.
//! 
//! When you list the analyses for a repository,
//! one or more will be identified as deletable in the response:
//! 
//! ```text
//! "deletable": true
//! ```
//! 
//! An analysis is deletable when it's the most recent in a set of analyses.
//! Typically, a repository will have multiple sets of analyses
//! for each enabled code scanning tool,
//! where a set is determined by a unique combination of analysis values:
//! 
//! * `ref`
//! * `tool`
//! * `analysis_key`
//! * `environment`
//! 
//! If you attempt to delete an analysis that is not the most recent in a set,
//! you'll get a 400 response with the message:
//! 
//! ```text
//! Analysis specified is not deletable.
//! ```
//! 
//! The response from a successful `DELETE` operation provides you with
//! two alternative URLs for deleting the next analysis in the set:
//! `next_analysis_url` and `confirm_delete_url`.
//! Use the `next_analysis_url` URL if you want to avoid accidentally deleting the final analysis
//! in a set. This is a useful option if you want to preserve at least one analysis
//! for the specified tool in your repository.
//! Use the `confirm_delete_url` URL if you are content to remove all analyses for a tool.
//! When you delete the last analysis in a set, the value of `next_analysis_url` and `confirm_delete_url`
//! in the 200 response is `null`.
//! 
//! As an example of the deletion process,
//! let's imagine that you added a workflow that configured a particular code scanning tool
//! to analyze the code in a repository. This tool has added 15 analyses:
//! 10 on the default branch, and another 5 on a topic branch.
//! You therefore have two separate sets of analyses for this tool.
//! You've now decided that you want to remove all of the analyses for the tool.
//! To do this you must make 15 separate deletion requests.
//! To start, you must find an analysis that's identified as deletable.
//! Each set of analyses always has one that's identified as deletable.
//! Having found the deletable analysis for one of the two sets,
//! delete this analysis and then continue deleting the next analysis in the set until they're all deleted.
//! Then repeat the process for the second set.
//! The procedure therefore consists of a nested loop:
//! 
//! **Outer loop**:
//! * List the analyses for the repository, filtered by tool.
//! * Parse this list to find a deletable analysis. If found:
//! 
//!   **Inner loop**:
//!   * Delete the identified analysis.
//!   * Parse the response for the value of `confirm_delete_url` and, if found, use this in the next iteration.
//! 
//! The above process assumes that you want to remove all trace of the tool's analyses from the GitHub user interface, for the specified repository, and it therefore uses the `confirm_delete_url` value. Alternatively, you could use the `next_analysis_url` value, which would leave the last analysis in each set undeleted to avoid removing a tool's analysis entirely.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/code-scanning#delete-a-code-scanning-analysis-from-a-repository)


fn url_string(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_analysis_id: i64,
    q_confirm_delete: ::std::option::Option<::std::option::Option<&str>>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 51);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/code-scanning/analyses/");
    ::querylizer::Simple::extend(&mut url, &p_analysis_id, false, &::querylizer::encode_path)?;
    if let Some(value) = &q_confirm_delete {
        url.push('?');
        ::querylizer::Form::extend(&mut url, "confirm_delete", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_analysis_id: i64,
    q_confirm_delete: ::std::option::Option<::std::option::Option<&str>>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_analysis_id,
        q_confirm_delete,
    )?;
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
    p_owner: &str,
    p_repo: &str,
    p_analysis_id: i64,
    q_confirm_delete: ::std::option::Option<::std::option::Option<&str>>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_analysis_id,
        q_confirm_delete,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
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
    p_owner: &str,
    p_repo: &str,
    p_analysis_id: i64,
    q_confirm_delete: ::std::option::Option<::std::option::Option<&str>>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_analysis_id,
        q_confirm_delete,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
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
