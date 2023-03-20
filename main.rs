use serde::{Deserialize, Serialize};
use std::{env, process};
use ureq;

fn main() {
    let args: Vec<String> = env::args().collect();
    let coin = coin_name(args).unwrap_or_else(|err| {
        eprintln!("{}\n Introduce a coin: \"cargo run bitcoin\"", err);
        process::exit(1);
    });
    let result_precio = get_precio(&coin).unwrap_or_else(|err| {
        eprintln!("Issue searching for the coin: {}", err);
        process::exit(1);
    });

    println!(
        "The {} price in USD is: ${}\nconverted to PEN: S/{}\nlast updated: {}",
        coin,
        result_precio.market_data.current_price.usd,
        result_precio.to_soles(),
        result_precio.market_data.last_updated
    )
}

impl CoinInfo {
    fn to_soles(&self) -> f64 {
        self.market_data.current_price.usd as f64
            * get_usd_price()
                .unwrap_or_else(|err| {
                    eprintln!("Issue searching for dollar exchange rate: {}", err);
                    process::exit(1);
                })
                .parse::<f64>()
                .unwrap()
    }
}

fn coin_name(args: Vec<String>) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("Not enough arguments");
    }
    let coin = args[1].clone();
    Ok(coin)
}

fn get_precio(coin: &str) -> Result<CoinInfo, ureq::Error> {
    let body: String = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false",
        coin
    ))
    .call()?
    .into_string()?;
    let coin_data: CoinInfo = serde_json::from_str(&body).unwrap();
    Ok(coin_data)
}

fn get_usd_price() -> Result<String, ureq::Error> {
    let body: String = ureq::get("https://api.apis.net.pe/v1/tipo-cambio-sunat")
        .call()?
        .into_string()?;
    let usd_price: ExchangeRate = serde_json::from_str(&body).unwrap();
    Ok(usd_price.venta.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinInfo {
    id: String,
    symbol: String,
    name: String,
    market_data: Market,
}

#[derive(Serialize, Deserialize, Debug)]
struct Market {
    current_price: Prices,
    last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExchangeRate {
    compra: f32,
    venta: f32,
}
