extern crate mongodb;
extern crate ring;
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use super::TransferInfo;

pub fn mongoinsert(txid : &str,fromAccount : & str,toAccount : & str,amount : & str,token : & str){

	let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("transfer");

    let doc = doc! {
        "fromAccount": fromAccount, 
        "toAccount": toAccount,
	"amount":amount,
	"token":token,
	"txid":txid,
    };
	

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok().expect("Failed to insert document.");

    // Find the document and receive a cursor
    let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");

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

fn account_send_receive(doc: & mongodb::ordered::OrderedDocument) -> Vec<TransferInfo> {
	let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("transfer");
	 let mut cursor = coll.find(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");


   let mut details = "".to_string();
      let mut data =Vec::new();

    for result in cursor {
	let mut details2 =  TransferInfo (
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
			if let Some(&Bson::String(ref amount)) = item.get("amount") {
			    let data = format!("amount: {}", amount);
			   details2.2 = data.to_string();
		       }
			if let Some(&Bson::String(ref token)) = item.get("token") {
			    let  data = format!("token: {}", token);
			   details2.3 = data.to_string();
		       }
			if let Some(&Bson::String(ref txid)) = item.get("txid") {
			    let data = format!("txid: {}", txid);
			   details2.4 = data.to_string();
		       }
	    }
	   data.push(details2);
    }
    data
}
pub fn account_history<'a>(account : &'a str) -> Vec<TransferInfo> {



	
	let doc_from = doc! {
        "fromAccount": account, 
    	};
	
	let doc_to = doc! {
        "toAccount": account, 
    	};
	
	let mut data_from = account_send_receive(&doc_from);
	let mut data_to = account_send_receive(&doc_to);

//  	println!("details={:?}",data);
  
  	data_from.extend_from_slice(&data_to[..]);
	data_from
}
