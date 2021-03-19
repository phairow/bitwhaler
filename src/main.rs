use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};
use reqwest::{ClientBuilder, redirect};
use serde::{Serialize, Deserialize};
use urlencoding::encode;

type Db = Arc<Mutex<HashMap<String, BitWhalerResponse>>>;

#[derive(Debug, Serialize)]
struct BitWhalerResponse {
    whale: BitWhalerWhale,
    greed: BitWhalerGreed,
}

#[derive(Debug, Serialize)]
struct BitWhalerWhale {
    blockchain: String,
    amount: String,
}

#[derive(Debug, Serialize)]
struct BitWhalerGreed {
    blockchain: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
struct WhaleTransactionAccount {
    address: String,
    owner_type: String,
}

#[derive(Debug, Deserialize)]
struct WhaleTransaction {
    blockchain: String,
    symbol: String,
    id: String,
    transaction_type: String,
    hash: String,
    from: WhaleTransactionAccount,
    to: WhaleTransactionAccount,
    timestamp: i64,
    amount: f64,
    amount_usd: f64,
    transaction_count: i32,
}

#[derive(Debug, Deserialize)]
struct WhaleResult {
    result: String,
    cursor: String,
    count: i64,
    #[serde(default = "Vec::new")]
    transactions: Vec<WhaleTransaction>
}   

#[tokio::main]
pub async fn main() {
    println!("Start Bit Whaler!");

    // store the results between
    let db = Arc::new(Mutex::new(HashMap::new()));
    db.lock().unwrap().insert("bitwhaler_result".to_string(), BitWhalerResponse{
        whale: BitWhalerWhale {
            blockchain: "".to_string(),
            amount: "".to_string(),
        },
        greed: BitWhalerGreed {
            blockchain: "".to_string(),
            amount: "".to_string(),
        }
    });
    
    // clones used in async loops
    let whale_db = db.clone();
    let greed_db = db.clone();

    // loop count for sanity check
    let mut whale_count: i64 = 0;
    let mut greed_count: i64 = 0;
    let mut process_count: i64 = 0;

    let diff = Duration::from_millis(10_000);
    let mut whale_time = SystemTime::now()
        .checked_sub(diff);

    tokio::spawn(async move {
        loop {
            println!("fetch_whale-{}", whale_count);
            whale_count += 1;

            let whale_db = whale_db.clone();
            fetch_whale(whale_db, whale_time.clone().unwrap()).await;
            let diff_add = Duration::from_millis(10_000);
            whale_time = whale_time.unwrap().checked_add(diff_add);
            sleep(Duration::from_millis(10_000)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            println!("fetch_greed-{}", greed_count);
            greed_count += 1;

            let greed_db = greed_db.clone();
            fetch_greed(greed_db).await;
            sleep(Duration::from_millis(3_600_000)).await;
        }
    });

    loop {
        println!("process-{}", process_count);
        process_count += 1;

        let db = db.clone();
        process(db).await;
        sleep(Duration::from_millis(5_000)).await;
    }
}

async fn process(db: Db) {
    let mut db = db.lock().unwrap();
    let mut bitwhaler_data;

    match db.get_mut("bitwhaler_result") {
        None => {
            println!("no value");
            return;
        },
        Some(value) => {
            bitwhaler_data = value;
        }
    }

    if bitwhaler_data.whale.blockchain.len() > 0 {
        let timeout = Duration::from_millis(10_000);
        let client = ClientBuilder::new().timeout(timeout).build().unwrap();
        let json_string = serde_json::to_string(&bitwhaler_data).unwrap();

        bitwhaler_data.whale = BitWhalerWhale{
            blockchain: "".to_string(),
            amount: "".to_string(),
        };

        let publish_data = encode(&json_string);
        let publish_uri = format!(
            "https://ps.pndsn.com/publish/pub-c-10921688-79ed-4759-b6e2-4388eed57ffe/sub-c-bc7c86ac-8ff9-11ea-9dd4-caf89c7998a9/0/whaler_process/0/{}",
            publish_data
        );
        let publish = client
            .get(publish_uri)
            .send()
            .await;
        
        let pubnub_body = publish.unwrap().text().await;
        println!("pubnub publish response: {:?}", pubnub_body)
    }
}

async fn fetch_whale(db: Db, time: SystemTime) {
    println!("-fetch_whale-");

    let timeout = Duration::from_millis(10000);
    let client = ClientBuilder::new()
        .timeout(timeout)
        .redirect(redirect::Policy::limited(20))
        .build()
        .unwrap();
    let diff_add = Duration::from_millis(10_000);
    let start_time = time
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let end_time = time.checked_add(diff_add)
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let uri = format!(
        "https://api.whale-alert.io/v1/transactions?start={}&end={}&limit=1&min_value=500000&api_key=aZgs2HiCOeTkvMDGn1ltbvp1IysxRiQF",
        start_time,
        end_time,
    );
    println!("URI: {}", uri);
    let resp = client
        .get(uri)
        .header("Cache-Control", "no-cache")
        .header("Host", "api.whale-alert.io")
        .header("User-Agent", "BitWhaler/1.0")
        .header("Accept","*/*")
        .send()
        .await;

    let resp_body = resp.unwrap().json().await;

    match resp_body {
        Ok(result) => {
            println!("Parsed JSON: {:?}", result);

            let whales: WhaleResult = result;

            if whales.transactions.len() > 0 {
                let mut db = db.lock().unwrap();
                let mut bitwhaler_result = db.get_mut("bitwhaler_result").unwrap();
                bitwhaler_result.whale.blockchain = whales.transactions[0].blockchain.to_string();
                bitwhaler_result.whale.amount = whales.transactions[0].amount.to_string();
            }
        },
        Err(err) => println!("error decoding response body {}", err)
    }
}

async fn fetch_greed(db: Db) {
    println!("-fetch_greed-");

    // let mut db = db.lock().unwrap();
    // db.insert("bitwhaler_result".to_string(), "hi there greed".to_string());
}
