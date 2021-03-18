extern crate hyper;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use hyper::{Client};

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
pub async fn main() {
    println!("Start Whaler!");

    let db = Arc::new(Mutex::new(HashMap::new()));

    let process_db = db.clone();
    let greed_db = db.clone();
    let whale_db = db.clone();
    let tickers_db = db.clone();

    tokio::spawn(async move {
        fetch_greed(greed_db).await;
    });

    tokio::spawn(async move {
        fetch_whale(whale_db).await;
    });

    tokio::spawn(async move {
        fetch_tickers(tickers_db).await;
    });

    process(process_db).await;
}

async fn process(db: Db) {
    println!("-process-");
    
    loop {
        let db = db.lock().unwrap();

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

        sleep(Duration::from_millis(100)).await;
    }
}

async fn fetch_greed(db: Db) {
    println!("-fetch_greed-");

    let mut db = db.lock().unwrap();

    db.insert("hig".to_string(), "hi there greed".to_string()); 
}

async fn fetch_whale(db: Db) {
    println!("-fetch_whale-");

    let client = Client::new();

    // let https = HttpsConnector::new();
    // let client = Client::builder().build::<_, hyper::Body>(https);
    

    let uri = "http://api.whale-alert.io/v1/transactions?limit=100&start=1616085702&min_value=500000&api_key=aZgs2HiCOeTkvMDGn1ltbvp1IysxRiQF".parse();
    let parsed_uri;

    match uri {
        Err(why) => {
            println!("fetch_whale::URI-ERROR {}", why);
            return;
        },
        Ok(value) => {
            parsed_uri = value;
        }
    }

    let resp = client.get(parsed_uri).await;

    let status;

    match resp {
        Err(why) => {
            println!("fetch_whale::status-error {}", why);
            return;
        },
        Ok(value) => {
            status = value;
        }
    }

    println!("Response: {:?}", status);

    let mut db = db.lock().unwrap();

    db.insert("hiw".to_string(), "hi there whale".to_string()); 
}

async fn fetch_tickers(db: Db) {
    println!("-fetch_tickers-");

    let mut db = db.lock().unwrap();

    db.insert("hit".to_string(), "hi there tickers".to_string()); 
}