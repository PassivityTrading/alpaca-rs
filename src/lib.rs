#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://files.alpaca.markets/webassets/favicon.ico")]
// perf
#![feature(async_fn_in_trait)]
#![deny(clippy::all)]
#![allow(
    // don't care
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    // required by alpaca api
    clippy::struct_excessive_bools,
    // allows for better dx
    clippy::wildcard_imports
)]
#![forbid(clippy::missing_safety_doc)]
// #![deny(missing_docs)]

endpoint_error!(Error);

use tracing::log::*;

use acril::prelude::http::*;
use base64::Engine;
use chrono::{NaiveDate, Utc};
use http_types::headers::AUTHORIZATION;

type Date = NaiveDate;
type DateTime<Tz = Utc> = chrono::DateTime<Tz>;
pub use chrono;

pub mod api;
pub mod model;
pub mod pagination;

/// An Alpaca [`Result`](core::result::Result).
/// This is just an alias to `Result<T, Error>`.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// An HTTP error (a "bad request" status code or a network error),
/// or a serialization error (which means the response from the API was not valid JSON).
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{_0}")]
    Http(http_types::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Query(#[from] acril::serde_urlencoded::ser::Error)
}

impl From<http_types::Error> for Error {
    fn from(value: http_types::Error) -> Self {
        Self::Http(value)
    }
}

/// `use alpaca_rs::prelude::*;` to import the most commonly used types and clients.
pub mod prelude {
    pub use crate::model::*;
    pub use crate::{
        api::broker::{BrokerAuth, BrokerClient},
        api::market_data::MarketDataClient,
        api::trading::{TradingAuth, TradingClient},
        Error as AlpacaError,
    };
}

/// A trait for identifying a single object in the API.
/// Required for pagination, as it needs the last item's ID.
pub trait Identifiable {
    fn id(&self) -> String;
    fn next_page_token(&self) -> Option<String> {
        Some(self.id())
    }
}
