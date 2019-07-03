extern crate mongodb;
extern crate ring;
use super::AccountInfo;
use super::Transaction;
use super::TransferInfo;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use mongodb::{Client, ThreadedClient};

pub fn transferinsert(txid: &str, fromAccount: &str, toAccount: &str, amount: &f64, token: &str) {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("transfer");

    let amount_clone = amount.clone();

    let doc = doc! {
        "fromAccount": fromAccount,
        "toAccount": toAccount,
    "amount":amount_clone,
    "token":token,
    "txid":txid,
    };

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok()
        .expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let item = cursor.next();
    match item {
        Some(Ok(doc)) => match doc.get("fromAccount") {
            Some(&Bson::String(ref title)) => println!("{}", title),
            _ => panic!("Expected title to be a string!"),
        },
        Some(Err(_)) => panic!("Failed to get next from server!"),
        None => panic!("Server returned no results!"),
    }
}

fn account_send_receive(doc: &mongodb::ordered::OrderedDocument) -> Vec<TransferInfo> {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("transfer");
    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let mut details = "".to_string();
    let mut data = Vec::new();

    for result in cursor {
        let mut details2 = TransferInfo(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );

        if let Ok(item) = result {
            if let Some(&Bson::String(ref fromAccount)) = item.get("fromAccount") {
                let data = format!("fromAccount: {}", fromAccount);
                details2.0 = data.to_string();
            }
            if let Some(&Bson::String(ref toAccount)) = item.get("toAccount") {
                let data = format!("toAccount: {}", toAccount);
                details2.1 = data.to_string();
            }
            if let Some(&Bson::FloatingPoint(ref amount)) = item.get("amount") {
                let data = format!("amount: {}", amount);
                details2.2 = data.to_string();
            }
            if let Some(&Bson::String(ref token)) = item.get("token") {
                let data = format!("token: {}", token);
                details2.3 = data.to_string();
            }
            if let Some(&Bson::String(ref txid)) = item.get("txid") {
                let data = format!("txid: {}", txid);
                details2.4 = data.to_string();
            }
        }

        println!("==============={:?}", details2);
        data.push(details2);
    }
    data
}
pub fn account_history<'a>(account: &'a str) -> Vec<TransferInfo> {
    let doc_from = doc! {
    "fromAccount": account,
    };

    let doc_to = doc! {
    "toAccount": account,
    };

    let mut data_from = account_send_receive(&doc_from);
    let mut data_to = account_send_receive(&doc_to);

    data_from.extend_from_slice(&data_to[..]);
    data_from
}

pub fn get_transaction_info(txid: &str) -> Vec<TransferInfo> {
    let doc = doc! {
    "txid": txid,
    };

    let mut transfer = account_send_receive(&doc);

    transfer
}

pub fn get_account_token_balance(account: &str, token: &str) -> f64 {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("account_info");

    let doc = doc! {
        "account": account,
    "token" : token,
    };
    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let mut balance: f64 = 0.0;

    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::FloatingPoint(amount)) = item.get("amount") {
                balance = amount;
            }
        }
    }
    balance
}

pub fn get_account_info(account: &str) -> Vec<AccountInfo> {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("account_info");

    let doc = doc! {
        "account": account,
    };
    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let mut details = "".to_string();
    let mut data = Vec::new();

    for result in cursor {
        let mut details2 = AccountInfo("".to_string(), "".to_string(), "".to_string());

        if let Ok(item) = result {
            if let Some(&Bson::String(ref account)) = item.get("account") {
                let data = format!("account: {}", account);
                details2.0 = data.to_string();
            }
            if let Some(&Bson::String(ref token)) = item.get("token") {
                println!("token: {}", token);
                let data = format!("token: {}", token);
                details2.1 = data.to_string();
            }

            if let Some(&Bson::FloatingPoint(ref amount)) = item.get("amount") {
                println!("amount: {}", amount);
                let data = format!("amount: {}", amount);
                details2.2 = data.to_string();
            }
        }

        data.push(details2);
    }
    data
}

pub fn update_account_info(account: &str, token: &str, amount: &f64) {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("account_info");
    let amount_clone = amount.clone();

    let doc = doc! {
        "account": account,
    "amount": amount_clone,
    "token":token,
    };
    if get_account_token_balance(account, token) == 0.0 {
        println!("ppp----{:?}", &doc);
        coll.insert_one(doc.clone(), None)
            .ok()
            .expect("Failed to insert document.");
    } else {
        let amount_filter = get_account_token_balance(account, token);
        let doc_filter = doc! {
                "account": account,
           "amount": amount_filter,
           "token":token,
        };
        let doc_update = doc! { "$set": {
                "account": account,
           "amount": amount_clone,
           "token":token,}
        };

        println!("ppp----{:?}", &doc_filter);
        coll.update_one(doc_filter, doc_update, None)
            .expect("Failed to update document.");
    }
}

pub fn update_key_info(private_key: &str, publish_key: &str, address: &str) {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("key_info");

    let doc = doc! {
        "private_key": private_key,
        "publish_key": publish_key,
    "address":address,
    };

    coll.insert_one(doc.clone(), None)
        .ok()
        .expect("Failed to insert document.");
}
pub fn get_private_key(address: &str) -> String {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("key_info");

    let doc = doc! {
        "address": address,
    };

    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let mut details = "".to_string();

    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref private_key)) = item.get("private_key") {
                details = private_key.to_string();
            }
        }
    }

    details
}

pub fn get_token_info(token_name: &str) -> bool {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("token_info");

    let doc = doc! {
        "token_name": token_name,
    };

    let mut valid = true;
    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let mut details = "".to_string();

    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref token)) = item.get("token_name") {
                details = token.to_string();
            }
        }
    }

    if details == "".to_string() {
        valid = false;
    }
    println!("valid={}", valid);
    valid
}

pub fn update_token_info(account: &str, token: &str, amount: &f64) {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("token_info");
    let amount_clone = amount.clone();

    let doc = doc! {
        "account": account,
    "amount": amount_clone,
    "token_name":token,
    };

    coll.insert_one(doc.clone(), None)
        .ok()
        .expect("Failed to insert document.");
}

pub fn find_official(official: &str) -> bool {
    let client =
        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("official");

    let doc = doc! {
        "official": official,
    };

    let mut valid = true;
    let mut cursor = coll
        .find(Some(doc.clone()), None)
        .ok()
        .expect("Failed to execute find.");

    let mut details = "".to_string();

    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref official)) = item.get("official") {
                details = official.to_string();
            }
        }
    }

    if details == "".to_string() {
        valid = false;
    }
    println!("valid={}", valid);
    valid
}
