extern crate mongodb;
extern crate ring;
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
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

pub fn account_history(fromAccount : & str){

	let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("exgpc").collection("transfer");

    // Find the document and receive a cursor
    let mut cursor = coll.find(None, None)
        .ok().expect("Failed to execute find.");
	println!("---{:?}---",cursor);
    for result in cursor {
	    if let Ok(item) = result {
		//if let Some(&Bson::String(ref title)) = item.get("toAccount") {
		    println!("title: {:?}", &item);
	       // }
	    }
    }

}
