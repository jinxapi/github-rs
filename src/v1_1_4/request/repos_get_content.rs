
//! Get repository content
//! 
//! Gets the contents of a file or directory in a repository. Specify the file path or directory in `:path`. If you omit
//! `:path`, you will receive the contents of the repository's root directory. See the description below regarding what the API response includes for directories. 
//! 
//! Files and symlinks support [a custom media type](https://docs.github.com/rest/reference/repos#custom-media-types) for
//! retrieving the raw content or rendered HTML (when supported). All content types support [a custom media
//! type](https://docs.github.com/rest/reference/repos#custom-media-types) to ensure the content is returned in a consistent
//! object format.
//! 
//! **Note**:
//! *   To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/reference/git#trees).
//! *   This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees
//! API](https://docs.github.com/rest/reference/git#get-a-tree).
//! *   This API supports files up to 1 megabyte in size.
//! 
//! #### If the content is a directory
//! The response will be an array of objects, one object for each item in the directory.
//! When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value
//! _should_ be "submodule". This behavior exists in API v3 [for backwards compatibility purposes](https://git.io/v1YCW).
//! In the next major version of the API, the type will be returned as "submodule".
//! 
//! #### If the content is a symlink 
//! If the requested `:path` points to a symlink, and the symlink's target is a normal file in the repository, then the
//! API responds with the content of the file (in the format shown in the example. Otherwise, the API responds with an object 
//! describing the symlink itself.
//! 
//! #### If the content is a submodule
//! The `submodule_git_url` identifies the location of the submodule repository, and the `sha` identifies a specific
//! commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out
//! the submodule at that specific commit.
//! 
//! If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the
//! github.com URLs (`html_url` and `_links["html"]`) will have null values.
//! 
//! [API method documentation](https://docs.github.com/rest/reference/repos#get-repository-content)


fn url_string(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_path: &str,
    q_ref: ::std::option::Option<&str>,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 37);
    url.push_str(trimmed);
    url.push_str("/repos/");
    ::querylizer::Simple::extend(&mut url, &p_owner, false, &::querylizer::encode_path)?;
    url.push('/');
    ::querylizer::Simple::extend(&mut url, &p_repo, false, &::querylizer::encode_path)?;
    url.push_str("/contents/");
    ::querylizer::Simple::extend(&mut url, &p_path, false, &::querylizer::encode_path)?;
    if let Some(value) = &q_ref {
        url.push('?');
        ::querylizer::Form::extend(&mut url, "ref", value, false, &::querylizer::encode_query)?;
    }
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_owner: &str,
    p_repo: &str,
    p_path: &str,
    q_ref: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_path,
        q_ref,
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
    p_path: &str,
    q_ref: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_path,
        q_ref,
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
    p_path: &str,
    q_ref: ::std::option::Option<&str>,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_owner,
        p_repo,
        p_path,
        q_ref,
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
