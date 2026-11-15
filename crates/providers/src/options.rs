use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Contract {
    /// Original provider fields for optional chain columns.
    pub fields: Value,
    pub symbol: String,
    pub kind: String,
    pub strike: f64,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub change_percentage: Option<f64>,
    pub volume: Option<f64>,
    pub open_interest: Option<f64>,
    pub iv: Option<f64>,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
}

fn items(value: &Value) -> Vec<&Value> {
    match value {
        Value::Array(a) => a.iter().collect(),
        Value::Object(_) | Value::String(_) => vec![value],
        _ => Vec::new(),
    }
}

fn number(value: &Value) -> Option<f64> {
    value.as_f64().filter(|v| v.is_finite())
}

fn parse_expirations(data: &Value) -> Vec<String> {
    items(&data["expirations"]["date"])
        .iter()
        .filter_map(|v| v.as_str())
        .filter(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok())
        .map(str::to_owned)
        .collect()
}

fn parse_chain(data: &Value) -> Vec<Contract> {
    items(&data["options"]["option"])
        .into_iter()
        .filter_map(|v| {
            let symbol = v["symbol"].as_str()?.to_owned();
            let kind = v["option_type"].as_str()?.to_owned();
            let strike = number(&v["strike"])?;
            if symbol.is_empty() || !matches!(kind.as_str(), "call" | "put") || strike <= 0.0 {
                return None;
            }
            let g = &v["greeks"];
            Some(Contract {
                fields: v.clone(),
                symbol,
                kind,
                strike,
                bid: number(&v["bid"]),
                ask: number(&v["ask"]),
                last: number(&v["last"]),
                change: number(&v["change"]),
                change_percentage: number(&v["change_percentage"]),
                volume: number(&v["volume"]),
                open_interest: number(&v["open_interest"]),
                iv: number(&g["mid_iv"]),
                delta: number(&g["delta"]),
                gamma: number(&g["gamma"]),
                theta: number(&g["theta"]),
                vega: number(&g["vega"]),
            })
        })
        .collect()
}

async fn get(
    client: &reqwest::Client,
    token: &str,
    path: &str,
    params: &[(&str, &str)],
) -> Result<Value, String> {
    let response = client
        .get(format!("https://api.tradier.com/v1/markets/options/{path}"))
        .bearer_auth(token)
        .header("Accept", "application/json")
        .query(params)
        .send()
        .await
        .map_err(|_| "Tradier options request failed".to_owned())?;
    if !response.status().is_success() {
        return Err(format!(
            "Tradier options unavailable (HTTP {}); check credentials, entitlements and rate limits",
            response.status()
        ));
    }
    super::market::read_json(response).await
}

pub async fn expirations(
    client: &reqwest::Client,
    token: &str,
    symbol: &str,
) -> Result<Vec<String>, String> {
    Ok(parse_expirations(
        &get(client, token, "expirations", &[("symbol", symbol)]).await?,
    ))
}

pub async fn chain(
    client: &reqwest::Client,
    token: &str,
    symbol: &str,
    expiration: &str,
) -> Result<Vec<Contract>, String> {
    Ok(parse_chain(
        &get(
            client,
            token,
            "chains",
            &[
                ("symbol", symbol),
                ("expiration", expiration),
                ("greeks", "true"),
            ],
        )
        .await?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn parses_single_and_multiple_results_without_inventing_values() {
        assert_eq!(
            parse_expirations(&json!({"expirations":{"date":"2027-01-15"}})),
            vec!["2027-01-15"]
        );
        let data = json!({"options":{"option":[{"symbol":"SPY270115C00500000","option_type":"call","strike":500.0,"bid":2.1,"greeks":{"delta":0.4}},{"symbol":"SPY270115P00500000","option_type":"put","strike":500.0}]}});
        let contracts = parse_chain(&data);
        assert_eq!(contracts.len(), 2);
        assert_eq!(contracts[0].delta, Some(0.4));
        assert_eq!(contracts[1].bid, None);
        assert!(parse_chain(&json!({"options":null})).is_empty());
    }

    #[cfg(feature = "vibestrader-credential")]
    #[test]
    #[ignore = "requires saved Tradier credential and network access"]
    fn live_spy_chain() {
        let token = crate::credentials::vibestrader_token()
            .or_else(|_| crate::credentials::tradecraft_token())
            .expect("saved credential");
        let client = crate::reference::client().unwrap();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let dates = rt.block_on(expirations(&client, &token, "SPY")).unwrap();
        let date = dates.first().expect("SPY expiration");
        let contracts = rt.block_on(chain(&client, &token, "SPY", date)).unwrap();
        assert!(contracts.iter().any(|c| c.kind == "call"));
        assert!(contracts.iter().any(|c| c.kind == "put"));
    }
}
