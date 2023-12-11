use std::time::Duration;

use actix::prelude::*;
use alpaca_rs::{
    api::{
        market_data::GetHistoricalBars,
        trading::{CancelOrder, CreateOrder, GetAccount, GetOpenPosition},
    },
    chrono::{self, Utc},
    model::{
        Account, OpenPosition, Order, OrderAmount, OrderClass, OrderSide, OrderTif, OrderType,
        SymbolOrAssetId, Timeframe,
    },
    Result, TradingAuth, TradingClient,
};
use tracing::*;
use tracing_subscriber::EnvFilter;

struct Service {
    alpaca: TradingClient,
    wait_for_open: bool,
    stock: String,
    last_order: Option<Order>,
    minutes: usize,
    running_avg: f64,
}

impl Service {
    async fn run(mut self) -> Result<()> {
        'main: loop {
            if self.wait_for_open {
                info!("Waiting for market opening...");
                self.alpaca.await_market_open().await?;
                info!("-- Market open!");
            } else {
                warn!("Not waiting for market open. Trades may not pass and/or profit.");
            }

            info!("-- Trading stock ${} --", self.stock);

            info!("Waiting for {} bars...", self.minutes);

            let now_date = alpaca_rs::chrono::Utc::now().naive_utc().date();

            while self
                .alpaca
                .execute_market(GetHistoricalBars {
                    symbols: vec![self.stock.to_owned()],
                    timeframe: Timeframe::Minutes(1),
                    limit: Some(self.minutes.try_into().unwrap()),
                    start: Some(now_date),
                    ..Default::default()
                })
                .await?
                .bars
                .get(&self.stock)
                .unwrap()
                .len()
                < self.minutes
            {
                actix_rt::time::sleep(Duration::from_secs(60)).await;
            }

            info!("Now we have {} bars.", self.minutes);

            loop {
                if let Some(order) = self.last_order.take() {
                    self.alpaca
                        .execute(CancelOrder { order_id: order.id })
                        .await?;
                }

                let interval = chrono::Duration::minutes(15);

                let time_to_close = self.alpaca.get_clock().await?.next_close;

                let till_close = time_to_close - Utc::now();
                if till_close < interval {
                    info!("Market closing soon. Liquidating positons.");

                    self.alpaca
                        .execute(CreateOrder {
                            symbol: self.stock.clone(),
                            side: OrderSide::Sell,
                            amount: OrderAmount::Quantity(
                                self.alpaca
                                    .execute(GetOpenPosition {
                                        symbol_or_asset_id: SymbolOrAssetId::SymbolId(
                                            self.stock.clone(),
                                        ),
                                    })
                                    .await?
                                    .qty,
                            ),
                            kind: OrderType::Market,
                            order_class: OrderClass::Simple,
                            time_in_force: OrderTif::Day,
                            extended_hours: false,
                            client_order_id: None,
                        })
                        .await?;

                    actix_rt::time::sleep(till_close.to_std().unwrap()).await;

                    info!("Market closed for this day. Waiting for the next trading day. Have a good night.");

                    continue 'main;
                } else {
                    self.rebalance().await?;
                }
            }

        }
    }

    async fn rebalance(&mut self) -> Result<()> {
        let OpenPosition {
            qty: pos_qty,
            market_value: pos_value,
            ..
        } = self
            .alpaca
            .execute(GetOpenPosition {
                symbol_or_asset_id: SymbolOrAssetId::SymbolId(self.stock.clone()),
            })
            .await?;

        let bars = self
            .alpaca
            .execute_market(GetHistoricalBars {
                symbols: vec![self.stock.clone()],
                limit: self.minutes.try_into().ok(),
                timeframe: Timeframe::Minutes(1),
                ..Default::default()
            })
            .await?
            .bars
            .remove(&self.stock)
            .unwrap();

        let current_price = bars.last().unwrap().closing_price;
        self.running_avg = 0.0;
        for bar in bars.iter() {
            self.running_avg += bar.closing_price;
        }
        self.running_avg /= bars.len() as f64;
        if current_price > self.running_avg {
            debug!("price was above running average, liquidating positions");
            // liquidate our position if the price is above the running averange
            if pos_qty > 0 {
                debug!("liquidating {pos_qty} positions at ${current_price} per share");

                self.last_order = Some(
                    self.alpaca
                        .execute(CreateOrder {
                            kind: OrderType::Limit {
                                limit_price: current_price,
                            },
                            symbol: self.stock.clone(),
                            side: OrderSide::Sell,
                            amount: OrderAmount::Quantity(pos_qty),
                            time_in_force: OrderTif::Day,
                            extended_hours: false,
                            client_order_id: None,
                            order_class: OrderClass::Simple,
                        })
                        .await?,
                );
            }
        } else if current_price < self.running_avg {
            let Account {
                portfolio_value,
                buying_power,
                ..
            } = self.alpaca.execute(GetAccount).await?;

            let portfolio_share = ((self.running_avg - current_price) / current_price) * 200.0;
            let target_position_value = portfolio_value * portfolio_share;
            let mut amount_to_add = target_position_value - pos_value;

            if amount_to_add > 0.0 {
                if amount_to_add > buying_power {
                    amount_to_add = buying_power;
                }
                let qty_to_buy = (amount_to_add / current_price).floor() as i64;
                self.last_order = Some(
                    self.alpaca
                        .execute(CreateOrder {
                            amount: OrderAmount::Quantity(qty_to_buy),
                            symbol: self.stock.clone(),
                            side: OrderSide::Buy,
                            kind: OrderType::Limit {
                                limit_price: current_price,
                            },
                            order_class: OrderClass::Simple,
                            client_order_id: None,
                            extended_hours: false,
                            time_in_force: OrderTif::Day,
                        })
                        .await?,
                );
            } else {
                amount_to_add *= -1.0;
                let qty_to_sell = ((amount_to_add / current_price).floor() as i64).max(pos_qty);
                self.last_order = Some(
                    self.alpaca
                        .execute(CreateOrder {
                            amount: OrderAmount::Quantity(qty_to_sell),
                            symbol: self.stock.clone(),
                            side: OrderSide::Sell,
                            kind: OrderType::Limit {
                                limit_price: current_price,
                            },
                            order_class: OrderClass::Simple,
                            client_order_id: None,
                            extended_hours: false,
                            time_in_force: OrderTif::Day,
                        })
                        .await?,
                );
            }
        }
        Ok(())
    }
}

#[actix_rt::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting mean-reversion algorithm");

    Service {
        alpaca: TradingClient::new_paper(TradingAuth {
            key: std::env::var("APCA_API_KEY").unwrap(),
            secret: std::env::var("APCA_SECRET_KEY").unwrap(),
        }),
        wait_for_open: !std::env::var("APCA_WAIT_OPEN").is_ok_and(|x| x == "0"),
        stock: std::env::var("APCA_MEANREV_STOCK").unwrap(),
        minutes: 20,
        last_order: None,
        running_avg: 0.0,
    }
    .run()
    .await?;

    System::current().stop();

    Ok(())
}
