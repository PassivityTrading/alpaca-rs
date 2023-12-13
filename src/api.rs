//! Modules implementing different Alpaca APIs.
//!
//! Currently provided are:
//! - the [Broker API](https://docs.alpaca.markets/docs/about-broker-api) - [`mod@broker`]
//! - the [Trading API](https://docs.alpaca.markets/docs/trading-api) - [`mod@trading`]
//! - (WIP) the [Market Data API](https://docs.alpaca.markets/docs/about-market-data-api) - [`mod@market_data`]
use super::*;
use crate::model::*;

pub mod broker;
pub mod market_data;
pub mod trading;

// hack: chrono is reexported from serde_with (we use a glob import) so we override that with an extern crate
extern crate chrono;

use serde::{Deserialize, Serialize};
use serde_with::{formats::*, *};
use std::collections::HashMap;
