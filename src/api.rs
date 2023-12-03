//! Modules implementing different Alpaca APIs.
//!
//! Currently provided are:
//! - the [Broker API](https://docs.alpaca.markets/docs/about-broker-api) - [`mod@broker`]
//! - the [Trading API](https://docs.alpaca.markets/docs/trading-api) - [`mod@trading`]
//! - (WIP) the [Market Data API](https://docs.alpaca.markets/docs/about-market-data-api) - [`mod@market_data`]
use super::*;
use crate::model::*;

pub mod broker;
pub mod trading;
pub mod market_data;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
