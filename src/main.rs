use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::{Result, Value};
use std::str::FromStr;
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let (mut socket, _) =
        connect(Url::parse("wss://ws.kraken.com").unwrap()).expect("Can't connect");

    println!("Connected to the server");

    socket
        .write_message(Message::Text("{\"event\":\"subscribe\", \"subscription\":{\"name\":\"ticker\"}, \"pair\":[\"XBT/USD\"]}".into()))
        .unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");

        log_message(&msg);
    }
}

fn log_message(msg: &Message) {
    let str_msg = msg.to_text().unwrap();
    let v: Value = serde_json::from_str(str_msg).unwrap();

    let data = match v.is_array() {
        // The message seems to be the ticker array. We want the second item
        true => v[1].clone(),
        false => {
            println!("Received {:?}", v);
            return;
        }
    };

    // let ticker: TickerLevels = serde_json::from_value(data).unwrap();
    let ticker: Ticker = serde_json::from_value(data).unwrap();

    // println!("TICKER RECEIVED, Best Ask: {:?}", ticker.ask);
    println!(
        "TICKER RECEIVED, Best Ask: {}",
        serde_json::to_string_pretty(&ticker).unwrap()
    );
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TickerLevels {
    #[serde(rename = "a")]
    ask: TickerLevelsDetail,

    #[serde(rename = "b")]
    bid: TickerLevelsDetail,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TickerLevelsDetail {
    price: String,

    whole_lot_volume: i64,

    lot_volume: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Ticker {
    #[serde(rename = "a")]
    ask: OrderLevel,

    #[serde(rename = "b")]
    bid: OrderLevel,

    #[serde(rename = "c")]
    close: TickerFloatValueToday,

    #[serde(rename = "v")]
    volume: TickerFloatValueToday,

    #[serde(rename = "p")]
    vwap: TickerFloatValueToday,

    #[serde(rename = "t")]
    trades: TickerIntValueToday,

    #[serde(rename = "l")]
    low: TickerFloatValueToday,

    #[serde(rename = "h")]
    high: TickerFloatValueToday,

    #[serde(rename = "o")]
    open: TickerFloatValueToday,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct OrderLevel {
    #[serde(deserialize_with = "de_f64_from_str")]
    price: f64,

    whole_lot_volume: i64,

    #[serde(deserialize_with = "de_f64_from_str")]
    lot_volume: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TickerPriceVolume {
    #[serde(deserialize_with = "de_f64_from_str")]
    price: f64,

    #[serde(deserialize_with = "de_f64_from_str")]
    lot_volume: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TickerIntValueToday {
    today: i64,
    last_24_hours: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct TickerFloatValueToday {
    #[serde(deserialize_with = "de_f64_from_str")]
    today: f64,

    #[serde(deserialize_with = "de_f64_from_str")]
    last_24_hours: f64,
}

fn de_f64_from_str<'de, D>(deserializer: D) -> std::result::Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    f64::from_str(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_json_test() {
        let json = r#"
        {
            "a": [
                "38784.90000",
                1,
                "1.34542395"
            ],
            "b": [
                "38784.80000",
                1,
                "1.54687685"
            ],
            "c": [
                "38784.90000",
                "0.00058989"
            ],
            "h": [
                "40222.40000",
                "40222.40000"
            ],
            "l": [
                "38251.50000",
                "38251.50000"
            ],
            "o": [
                "39435.80000",
                "39483.10000"
            ],
            "p": [
                "38998.32900",
                "38998.43866"
            ],
            "t": [
                28980,
                29021
            ],
            "v": [
                "3485.87146780",
                "3486.74306333"
            ]
        }"#;

        let ticker: Ticker = serde_json::from_str(json).unwrap();

        println!("{}", serde_json::to_string_pretty(&ticker).unwrap());
    }
}
