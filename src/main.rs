use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
pub async fn main() {
    println!("Start Whaler!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    let process_db = db.clone();

    process(process_db).await;
}

async fn process(db: Db) {
    println!("-process-");

    loop {
        let greed_db = db.clone();
        let whale_db = db.clone();
        let tickers_db = db.clone();

        let db = db.lock().unwrap();


        tokio::spawn(async move {
            fetch_greed(greed_db).await;
        });

        tokio::spawn(async move {
            fetch_whale(whale_db).await;
        });

        tokio::spawn(async move {
            fetch_tickers(tickers_db).await;
        });

        match db.get("hig") {
            None => println!("no value"),
            Some(value) => {
                println!("value: {}", value)
            }
        }

        match db.get("hiw") {
            None => println!("no value"),
            Some(value) => {
                println!("value: {}", value)
            }
        }

        match db.get("hit") {
            None => println!("no value"),
            Some(value) => {
                println!("value: {}", value)
            }
        }

        sleep(Duration::from_millis(4000)).await;
    }
}

async fn fetch_greed(db: Db) {
    println!("-fetch_greed-");

    let mut db = db.lock().unwrap();

    db.insert("hig".to_string(), "hi there greed".to_string()); 
}

async fn fetch_whale(db: Db) -> Result<(), Box<dyn std::error::Error>> {
    println!("-fetch_whale-");

    let resp = reqwest::get("https://api.whale-alert.io/v1/transactions?limit=100&min_value=500000&api_key=aZgs2HiCOeTkvMDGn1ltbvp1IysxRiQF")
        .await?;


        // .json::<HashMap<String, String>>()
        // .await?;

    println!("Response: {:?}", resp);
    let parsed = resp.json::<HashMap<String, String>>().await;
    println!("{:?}", parsed);

    let mut db = db.lock().unwrap();

    db.insert("hiw".to_string(), "hi there whale".to_string());

    Ok(())
}

async fn fetch_tickers(db: Db) {
    println!("-fetch_tickers-");

    let mut db = db.lock().unwrap();

    db.insert("hit".to_string(), "hi there tickers".to_string()); 
}
