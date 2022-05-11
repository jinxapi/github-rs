//! Create or update an environment secret
//! 
//! Creates or updates an environment secret with an encrypted value. Encrypt your secret using
//! [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access
//! token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use
//! this endpoint.
//! 
//! #### Example encrypting a secret using Node.js
//! 
//! Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
//! 
//! ```text
//! const sodium = require('tweetsodium');
//! 
//! const key = "base64-encoded-public-key";
//! const value = "plain-text-secret";
//! 
//! // Convert the message and key to Uint8Array's (Buffer implements that interface)
//! const messageBytes = Buffer.from(value);
//! const keyBytes = Buffer.from(key, 'base64');
//! 
//! // Encrypt using LibSodium.
//! const encryptedBytes = sodium.seal(messageBytes, keyBytes);
//! 
//! // Base64 the encrypted secret
//! const encrypted = Buffer.from(encryptedBytes).toString('base64');
//! 
//! console.log(encrypted);
//! ```
//! 
//! 
//! #### Example encrypting a secret using Python
//! 
//! Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/latest/public/#nacl-public-sealedbox) with Python 3.
//! 
//! ```text
//! from base64 import b64encode
//! from nacl import encoding, public
//! 
//! def encrypt(public_key: str, secret_value: str) -> str:
//!   """Encrypt a Unicode string using the public key."""
//!   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
//!   sealed_box = public.SealedBox(public_key)
//!   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
//!   return b64encode(encrypted).decode("utf-8")
//! ```
//! 
//! #### Example encrypting a secret using C#
//! 
//! Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
//! 
//! ```text
//! var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
//! var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
//! 
//! var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
//! 
//! Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
//! ```
//! 
//! #### Example encrypting a secret using Ruby
//! 
//! Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
//! 
//! ```ruby
//! require "rbnacl"
//! require "base64"
//! 
//! key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
//! public_key = RbNaCl::PublicKey.new(key)
//! 
//! box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
//! encrypted_secret = box.encrypt("my_secret")
//! 
//! # Print the base64 encoded secret
//! puts Base64.strict_encode64(encrypted_secret)
//! ```
//! 
//! [API method documentation](https://docs.github.com/rest/reference/actions#create-or-update-an-environment-secret)

pub struct Content<Body>
{
    body: Body,
    content_type_value: Option<::std::borrow::Cow<'static, [u8]>>,
}

impl<Body> Content<Body> {
    pub fn new(body: Body) -> Self {
        Self { body, content_type_value: None }
    }

    #[must_use]
    pub fn with_content_type(mut self, content_type: impl Into<::std::borrow::Cow<'static, [u8]>>) -> Self {
        self.content_type_value = Some(content_type.into());
        self
    }

    fn content_type(&self) -> Option<&[u8]> {
        self.content_type_value.as_deref()
    }

    fn into_body(self) -> Body {
        self.body
    }
}

fn url_string(
    base_url: &str,
    p_repository_id: i64,
    p_environment_name: &str,
    p_secret_name: &str,
) -> Result<String, crate::v1_1_4::ApiError> {
    let trimmed = if base_url.is_empty() {
        "https://api.github.com"
    } else {
        base_url.trim_end_matches('/')
    };
    let mut url = String::with_capacity(trimmed.len() + 56);
    url.push_str(trimmed);
    url.push_str("/repositories/");
    ::querylizer::Simple::extend(&mut url, &p_repository_id, false, &::querylizer::encode_path)?;
    url.push_str("/environments/");
    ::querylizer::Simple::extend(&mut url, &p_environment_name, false, &::querylizer::encode_path)?;
    url.push_str("/secrets/");
    ::querylizer::Simple::extend(&mut url, &p_secret_name, false, &::querylizer::encode_path)?;
    Ok(url)
}

#[cfg(feature = "hyper")]
pub fn http_builder(
    base_url: &str,
    p_repository_id: i64,
    p_environment_name: &str,
    p_secret_name: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::http::request::Builder, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_repository_id,
        p_environment_name,
        p_secret_name,
    )?;
    let mut builder = ::http::request::Request::put(url);
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
pub fn hyper_request(
    mut builder: ::http::request::Builder,
    content: Content<::hyper::Body>,
) -> Result<::http::request::Request<::hyper::Body>, crate::v1_1_4::ApiError>
{
    if let Some(content_type) = content.content_type() {
        builder = builder.header(::http::header::CONTENT_TYPE, content_type);
    }
    Ok(builder.body(content.into_body())?)
}

#[cfg(feature = "hyper")]
impl From<::hyper::Body> for Content<::hyper::Body> {
    fn from(body: ::hyper::Body) -> Self {
        Self::new(body)
    }
}

#[cfg(feature = "reqwest")]
pub fn reqwest_builder(
    base_url: &str,
    p_repository_id: i64,
    p_environment_name: &str,
    p_secret_name: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_repository_id,
        p_environment_name,
        p_secret_name,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::Request::new(::reqwest::Method::PUT, reqwest_url);
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
pub fn reqwest_request(
    mut builder: ::reqwest::Request,
    content: Content<::reqwest::Body>,
) -> Result<::reqwest::Request, crate::v1_1_4::ApiError> {
    if let Some(content_type) = content.content_type() {
        builder.headers_mut().append(
            ::reqwest::header::HeaderName::from_static("content-type"),
            ::reqwest::header::HeaderValue::try_from(content_type)?,
        );
    }
    *builder.body_mut() = Some(content.into_body());
    Ok(builder)
}

#[cfg(feature = "reqwest")]
impl From<::reqwest::Body> for Content<::reqwest::Body> {
    fn from(body: ::reqwest::Body) -> Self {
        Self::new(body)
    }
}

#[cfg(feature = "reqwest-blocking")]
pub fn reqwest_blocking_builder(
    base_url: &str,
    p_repository_id: i64,
    p_environment_name: &str,
    p_secret_name: &str,
    h_user_agent: &str,
    h_accept: ::std::option::Option<&str>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    let url = url_string(
        base_url,
        p_repository_id,
        p_environment_name,
        p_secret_name,
    )?;
    let reqwest_url = ::reqwest::Url::parse(&url)?;
    let mut request = ::reqwest::blocking::Request::new(::reqwest::Method::PUT, reqwest_url);
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
pub fn reqwest_blocking_request(
    mut builder: ::reqwest::blocking::Request,
    content: Content<::reqwest::blocking::Body>,
) -> Result<::reqwest::blocking::Request, crate::v1_1_4::ApiError> {
    if let Some(content_type) = content.content_type() {
        builder.headers_mut().append(
            ::reqwest::header::HeaderName::from_static("content-type"),
            ::reqwest::header::HeaderValue::try_from(content_type)?,
        );
    }
    *builder.body_mut() = Some(content.into_body());
    Ok(builder)
}

#[cfg(feature = "reqwest-blocking")]
impl From<::reqwest::blocking::Body> for Content<::reqwest::blocking::Body> {
    fn from(body: ::reqwest::blocking::Body) -> Self {
        Self::new(body)
    }
}

/// Types for body parameter in [`super::actions_create_or_update_environment_secret`]
pub mod body {
    #[allow(non_snake_case)]
    #[derive(Clone, Eq, PartialEq, Debug, Default, ::serde::Serialize, ::serde::Deserialize)]
    pub struct Json<'a> {
        /// Value for your secret, encrypted with [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages) using the public key retrieved from the [Get an environment public key](https://docs.github.com/rest/reference/actions#get-an-environment-public-key) endpoint.
        pub encrypted_value: ::std::borrow::Cow<'a, str>,

        /// ID of the key you used to encrypt the secret.
        pub key_id: ::std::borrow::Cow<'a, str>,

        #[serde(flatten)]
        pub additionalProperties: ::std::collections::HashMap<::std::borrow::Cow<'a, str>, ::serde_json::value::Value>
    }

    #[cfg(feature = "hyper")]
    impl<'a> TryFrom<&Json<'a>> for super::Content<::hyper::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &Json<'a>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }

    #[cfg(feature = "reqwest")]
    impl<'a> TryFrom<&Json<'a>> for super::Content<::reqwest::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &Json<'a>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }

    #[cfg(feature = "reqwest-blocking")]
    impl<'a> TryFrom<&Json<'a>> for super::Content<::reqwest::blocking::Body> {
        type Error = crate::v1_1_4::ApiError;

        fn try_from(value: &Json<'a>) -> Result<Self, Self::Error> {
            Ok(
                Self::new(::serde_json::to_vec(value)?.into())
                .with_content_type(&b"application/json"[..])
            )
        }
    }
}
