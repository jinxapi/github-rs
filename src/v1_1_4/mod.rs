
//! GitHub v3 REST API
//! 
//! 1.1.4
//! 
//! GitHub's v3 REST API.
//! 
//! [GitHub v3 REST API](https://docs.github.com/rest/)

use thiserror::Error;

mod support;

pub mod config;
pub mod request;
pub mod schema;

#[cfg(feature = "hyper")]
pub mod hyper;

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[derive(Error, Debug)]
pub enum ApiError {
    #[cfg(feature = "hyper")]
    #[error("Hyper error")]
    Hyper {
        #[from]
        source: ::hyper::Error,
    },

    #[cfg(feature = "reqwest")]
    #[error("Reqwest error")]
    Reqwest {
        #[from]
        source: ::reqwest::Error,
    },

    #[cfg(feature = "reqwest")]
    #[error("URL parse error")]
    UrlParser {
        #[from]
        source: ::url::ParseError,
    },

    #[error("AuthenticError")]
    Authentic {
        #[from]
        source: ::authentic::AuthenticError,
    },

    #[error("QuerylizerError")]
    Querylizer {
        #[from]
        source: ::querylizer::QuerylizerError,
    },

    #[error("InvalidHeaderValue")]
    InvalidHeaderValue {
        #[from]
        source: ::hyper::header::InvalidHeaderValue,
    },
    #[error("HttpError")]
    Http {
        #[from]
        source: ::http::Error,
    },
    #[error("JsonSerializationError")]
    JsonSerializationError {
        #[from]
        source: ::serde_json::error::Error,
    },
    #[error("FormatError")]
    Format {
        #[from]
        source: ::std::fmt::Error,
    },
    #[error("{0}")]
    Other(String),
}