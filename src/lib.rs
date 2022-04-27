//! Clients for GitHub API.
//!
//! # HTTP library support
//!
//! The underlying web library can be selected as a feature.
//!
//! For `hyper`, the feature `hyper-client` is enabled by default.
//!
//! For async `reqwest` use
//! ```text
//! [dependencies]
//! jinxapi-github = { version = "0.1", default-features = false, features = ["reqwest-async"] }
//! ```
//!
//! For blocking `reqwest` use
//! ```text
//! [dependencies]
//! jinxapi-github = { version = "0.1", default-features = false, features = ["reqwest-blocking"] }
//! ```
//!
//! # Using the API
//!
//! Broadly, the steps to use the API are:
//! - Create a [`v1_1_4::config::Configuration`] object, including authentication credentials.
//! - Create a client for the underlying HTTP library (`hyper` or `reqwest`).
//! - Create a `Caller` ([`v1_1_4::hyper::Caller`], [`v1_1_4::reqwest::Caller`], [`v1_1_4::reqwest::blocking::Caller`]), providing the `Configuration`, Client, and an appropriate function
//! for sleeping (e.g. `std::thread::sleep` or `tokio::time::sleep`).
//! - For requests with a body, create the body type.
//! - Call a method on the `Caller`, passing in operation parameters and, if required, a body.
//! - Handle the response returned from the method.
//!
//! See the [test](https://github.com/jinxapi/github-rs/tree/main/tests) and [example](https://github.com/jinxapi/github-rs/tree/main/examples) code for examples of these steps.



pub mod types;

pub mod v1_1_4;
