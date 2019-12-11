use std::process::Command;
extern crate rustc_serialize;
// 引入rustc_serialize模块
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use rustc_serialize::json;
use serde::Deserialize;

extern crate mongodb;
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};
use mongodb::{bson, doc, Bson};
use mongodb::coll::options::{FindOptions, FindOneAndUpdateOptions,ReturnDocument};
use nix::unistd::{fork, ForkResult};
use std::{thread, time};
use std::sync::Mutex;
extern crate rust_decimal;
use rust_decimal::Decimal;
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;


lazy_static! {
        static ref CLIENTDB: Mutex<mongodb::Client> = Mutex::new({
		 let client = Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");
            	 client
        });
}


struct BaseSprite;
#[derive(RustcDecodable, RustcEncodable)]
struct BlockStruct {
    hash: String,
    confirmations: u32,
    size: u32,
    height: u32,
    version: u32,
    merkleroot: String,
    tx: Vec<String>,
    time: u32,
    nonce: u32,
    bits: String,
    difficulty: f32,
    chainwork: String,
    previousblockhash: String,
    nextblockhash: String,
}

/**
{
    "jsonrpc": "2.0",
    "result": {
        "author": "0x00cfaddcdcd6575c2528f0bd3a7ce84dcb169019",
        "difficulty": "0x37663b1a",
        "extraData": "0xd5830104058650617269747986312e31332e30827769",
        "gasLimit": "0x47b760",
        "gasUsed": "0x5208",
        "hash": "0x10c831418e87185db0dba3e617ddf52a96fe658b2606fd9db026d229eccfb0eb",
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "miner": "0x00cfaddcdcd6575c2528f0bd3a7ce84dcb169019",
        "mixHash": "0xc945ac7f63c6f6d1bc9096f5c67693f998df413cd7ddbf2e9bc223358039f830",
        "nonce": "0xdda7635dbc12dfc4",
        "number": "0x1b444",
        "parentHash": "0x73c62809cdb66085d86dfb9cda7b2a70b85870afcc708b451430ec0516f25981",
        "receiptsRoot": "0x143ffab97e97651892106d5e77142f7af8127914affa62f5bc0a522437528575",
        "sealFields": ["0xa0c945ac7f63c6f6d1bc9096f5c67693f998df413cd7ddbf2e9bc223358039f830", "0x88dda7635dbc12dfc4"],
        "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
        "size": "0x28d",
        "stateRoot": "0xfe37b985fcc427a98e4309e5f3ebff269c9ddad48c65af3e3b043ed52f5afb0d",
        "timestamp": "0x58478161",
        "totalDifficulty": "0x104ae2a5dcfc",
        "transactions": [{
            "blockHash": "0x10c831418e87185db0dba3e617ddf52a96fe658b2606fd9db026d229eccfb0eb",
            "blockNumber": "0x1b444",
            "chainId": null,
            "condition": null,
            "creates": null,
            "from": "0x687422eea2cb73b5d3e242ba5456b782919afc85",
            "gas": "0x4cb26",
            "gasPrice": "0x4a817c800",
            "hash": "0xd9c90542f8a34b02f9a03b764dfc13fe7b6bd53bdeb0e56e97a45c21e52c9d39",
            "input": "0x",
            "nonce": "0x5004",
            "publicKey": "0x0bd518dd837e6ed3b902452c0075a4f8d09c8a194cf0ecb8012ca419b6f13916ca560cc840413edcd8cd91c43ca6d86a2d1e8b0bd1bb5fa2c35044fbb42a3cd1",
            "r": "0xbbac38daf4b2591a86fd5e6d09aa114c0d79c57a3f7f17f9ad9317f152f67349",
            "raw": "0xf86f8250048504a817c8008304cb269432cd3282d33ff58b4ae8402a226a0b27441b7f1a880de0b6b3a7640000801ca0bbac38daf4b2591a86fd5e6d09aa114c0d79c57a3f7f17f9ad9317f152f67349a06526acc61a394988baaca82c37d41589ffe60d4d91475c1c000f2cd07a1d22e0",
            "s": "0x6526acc61a394988baaca82c37d41589ffe60d4d91475c1c000f2cd07a1d22e0",
            "standardV": "0x1",
            "to": "0x32cd3282d33ff58b4ae8402a226a0b27441b7f1a",
            "transactionIndex": "0x0",
            "v": "0x1c",
            "value": "0xde0b6b3a7640000"
        }],
        "transactionsRoot": "0x98fb2df6bc6d138ccc668dc2c0426d6284e8d5fadf965babbf71d48f73f9df93",
        "uncles": []
    },
    "id": 1
}
**/

trait OpertionMongo {
    fn insert(&self);
    fn insert_txs(&self);
    fn filter_usdt(&self);
}

#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct BaseInfo {}
#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default,Clone)]
struct Transaction {
    blockHash: String,
    blockNumber: String,
    chainId: String,
    condition: String,
    creates: String,
    from: String,
    gas: String,
    gasPrice: String,
    hash: String,
    input: String,
    nonce: String,
    publicKey: String,
    r: String,
    raw: String,
    s: String,
    standardV: String,
    to: String,
    transactionIndex: String,
    v: String,
    value: String,
}
#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default,Clone)]
struct EthBlockStruct {
    author: String,
    difficulty: String,
    extraData: String,
    gasLimit: String,
    gasUsed: String,
    hash: String,
    logsBloom: String,
    miner: String,
    mixHash: String,
    nonce: String,
    number: String,
    parentHash: String,
    receiptsRoot: String,
    sealFields: Vec<String>,
    sha3Uncles: String,
    size: String,
    stateRoot: String,
    timestamp: String,
    totalDifficulty: String,
    transactions: Vec<Transaction>,
    transactionsRoot: String,
    uncles: Vec<String>,
}

#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct ReturnBlock {
    result: EthBlockStruct,
    jsonrpc: String,
    id: u32,
}

fn string_info(output: Vec<u8>) -> String {
    let mut one = output;
    one.reverse();
    let mut issue_result: String = "".to_string();
    while let Some(top) = one.pop() {
        issue_result += &(top as char).to_string();
    }
    let result = str::replace(&issue_result, "null", "\"null\"");
    result
}

/*
{
    "jsonrpc": "2.0", 
    "id": 1, 
    "result": {
        "blockHash": "0xb9d6465f65ecbc8f1d1f7e955652a7bda1154a9b88a91011815be8a014d0666a", 
        "blockNumber": "0x672ef1", 
        "contractAddress": null, 
        "cumulativeGasUsed": "0x4890e5", 
        "from": "0xf7ad9e873ed1c6d257c7d497d78272e7f3574aa4", 
        "gasUsed": "0xdbb2", 
        "logs": [
            {
                "address": "0x2482a9c2573b13f70413030004f76b1421749d44", 
                "blockHash": "0xb9d6465f65ecbc8f1d1f7e955652a7bda1154a9b88a91011815be8a014d0666a", 
                "blockNumber": "0x672ef1", 
                "data": "0x000000000000000000000000000000000000000000000006aaf7c8516d0c0000", 
                "logIndex": "0x1c", 
                "removed": false, 
                "topics": [
                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef", 
                    "0x000000000000000000000000f7ad9e873ed1c6d257c7d497d78272e7f3574aa4", 
                    "0x000000000000000000000000b1648746dabfc8e8c920f57f8b445bc08d3e6675"
                ], 
                "transactionHash": "0x66649913b6a26c3df47eb95ccf0185e16f19922bbc2a2e66381eab8704b60c80", 
                "transactionIndex": "0x49"
            }
        ], 
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000020000000000000000100000000000000000000000000000000000000000000000000000000000000000000001000008000000000000080040000000000000000000000000000000000000000000000000000000000000000000000000000010000000400000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000010000", 
        "status": "0x1", 
        "to": "0x2482a9c2573b13f70413030004f76b1421749d44", 
        "transactionHash": "0x66649913b6a26c3df47eb95ccf0185e16f19922bbc2a2e66381eab8704b60c80", 
        "transactionIndex": "0x49"
    }
}
*/

#[derive(RustcDecodable, RustcEncodable, Deserialize,PartialEq, Debug, Default)]
struct Log{
	address:String,
	blockHash:String,
	blockNumber:String,
	data:String,
	logIndex:String,
	removed:bool,
	topics:Vec<String>,
	transactionHash:String,
	transactionIndex:String
}

#[derive(RustcDecodable, RustcEncodable, PartialEq,Deserialize, Debug, Default)]
struct ReceiptResult{
	blockHash:String,
	blockNumber:String,
	contractAddress:String,
	cumulativeGasUsed:String,
	from:String,
	gasUsed:String,
	logs:Vec<Log>,
	logsBloom:String,
	status:String,
	to:String,
	transactionHash:String,
	transactionIndex:String
}

#[derive(RustcDecodable, RustcEncodable, PartialEq,Deserialize, Debug, Default)]
struct Receipt{
    result: ReceiptResult,
    jsonrpc: String,
    id: u32,
}

fn get_receipt(txid:&str) -> Receipt{
    //let txid2 = "0x66649913b6a26c3df47eb95ccf0185e16f19922bbc2a2e66381eab8704b60c80";	
   // let txid2 = "0x66649913b6a26c3df47eb95ccf0185e16f19922bbc2a2e66381eab8704b60c80";	
    let data = format! {"{{\"method\":\"eth_getTransactionReceipt\",\"params\":[\"{}\"],\"id\":1,\"jsonrpc\":\"2.0\"}}",txid};
    let list_dir = Command::new("curl")
        .args(&[
            "--data",
            &data,
            "-H",
            "Content-Type: application/json",
            "-X",
            "POST",
            "http://172.18.185.144:29842",
        ])
        .output()
        .expect("ls command failed to start");
    println!("88888-2{:?}", list_dir);
    let info = string_info(list_dir.stdout);
    println!("88888-2{:?}", info);
   // let decoded:Receipt = json::decode(&info);
    let mut decoded:Receipt = Default::default(); 
    if let Ok(tmp) = json::decode(&info){
            decoded = tmp;
    }else{
	println!("this is not a erc20 transfer");
    }
    decoded
}

impl OpertionMongo for ReturnBlock {
    fn insert(&self) {
      //  let client =
    //        Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

        let coll = CLIENTDB.lock().unwrap().db("eth").collection("block");
        //let block_height:u32 = self.result.number.clone().parse().unwrap();
	//let block_height = u32::from_str_radix(&self.result.number, 16).unwrap();
	//let block_height = u64::from_str_radix(&self.result.number.get(2..).unwrap(), 16).unwrap();
	let block_height = self.result.number.clone();
        let block_hash = self.result.hash.clone();
        let txids = format!("{:?}", self.result.transactions);
        let date = self.result.timestamp.clone();

      //  println!("11111{}--22{:?}", txids, self.result.transactions);
        let doc = doc! {
        "blockheight": &block_height,
        "blockhash": &block_hash,
        "txids": &txids,
        "date": &date,
        };

        // Insert document into 'test.movies' collection
        coll.insert_one(doc.clone(), None)
            .ok()
            .expect("Failed to insert document. returnblock");
	println!("doc=doc{:?}",doc);
    }


    fn insert_txs(&self) {
        //let client =
         //   Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

        let coll = CLIENTDB.lock().unwrap().db("eth").collection("transactions");
	//库里没有u128
	//let amount  = u64::from_str_radix(&self.value.get(2..).unwrap(), 16).unwrap();
	let mut docs = vec![];
	let txs = &self.result.transactions;
	for transaction in txs {
		let doc = doc! {
		    "blockHash":transaction.blockHash.clone(),
		    "blockNumber":transaction.blockNumber.clone(),
		    "chainId":transaction.chainId.clone(),
		    "condition":transaction.condition.clone(),
		"creates":transaction.creates.clone(),
		"from":transaction.from.clone(),
		"gas":transaction.gas.clone(),
		"gasPrice":transaction.gasPrice.clone(),
		"hash":transaction.hash.clone(),
		"input":transaction.input.clone(),
		"nonce":transaction.nonce.clone(),
		"publicKey":transaction.publicKey.clone(),
		"r":transaction.r.clone(),
		"raw":transaction.raw.clone(),
		"s":transaction.s.clone(),
		"standardV":transaction.standardV.clone(),
		"to":transaction.to.clone(),
		"transactionIndex":transaction.transactionIndex.clone(),
		"v":transaction.v.clone(),
		"value":transaction.value.clone()
		};
		docs.push(doc.clone());	
	}
        // Insert document into 'test.movies' collection
	//println!("----insert--transaction---{}",docs);
        coll.insert_many(docs, None)
            .ok()
            .expect("Failed to insert document.transaction");
    }
/*
{
    "jsonrpc": "2.0", 
    "id": 1, 
    "result": {
        "blockHash": "0xb9d6465f65ecbc8f1d1f7e955652a7bda1154a9b88a91011815be8a014d0666a", 
        "blockNumber": "0x672ef1", 
        "contractAddress": null, 
        "from": "0xf7ad9e873ed1c6d257c7d497d78272e7f3574aa4", 
        "gasUsed": "0xdbb2", 
        "logs": [
            {
                "address": "0x2482a9c2573b13f70413030004f76b1421749d44", 
                "blockHash": "0xb9d6465f65ecbc8f1d1f7e955652a7bda1154a9b88a91011815be8a014d0666a", 
                "blockNumber": "0x672ef1", 
                "data": "0x000000000000000000000000000000000000000000000006aaf7c8516d0c0000", 
                "logIndex": "0x1c", 
                "removed": false, 
                "topics": [
                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef", 
                    "0x000000000000000000000000f7ad9e873ed1c6d257c7d497d78272e7f3574aa4", 
                    "0x000000000000000000000000b1648746dabfc8e8c920f57f8b445bc08d3e6675"
                ], 
                "transactionHash": "0x66649913b6a26c3df47eb95ccf0185e16f19922bbc2a2e66381eab8704b60c80", 
                "transactionIndex": "0x49"
            }
        ], 
        "logsBloom": "0x00000000000000000000000000000000000000000000000000000000020000000000000000100000000000000000000000000000000000000000000000000000000000000000000001000008000000000000080040000000000000000000000000000000000000000000000000000000000000000000000000000010000000400000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000010000", 
        "to": "0x2482a9c2573b13f70413030004f76b1421749d44", 
        "transactionHash": "0x66649913b6a26c3df47eb95ccf0185e16f19922bbc2a2e66381eab8704b60c80", 
        "transactionIndex": "0x49"
*/
    fn filter_usdt(&self){
        let coll = CLIENTDB.lock().unwrap().db("eth").collection("erc20");
	//库里没有u128
	//let amount  = u64::from_str_radix(&self.value.get(2..).unwrap(), 16).unwrap();
	let txs = &self.result.transactions;

       	let mut docs = vec![];
	for transaction in txs {
	
	   	let receipt_info = get_receipt(&transaction.hash);
	    	if receipt_info != Receipt::default() && receipt_info.result.to == "0x2482a9c2573b13f70413030004f76b1421749d44"{
		println!("find a usdt transfer");

			let doc = doc! {
			 "blockHash":receipt_info.result.blockHash,
			"blockNumber":receipt_info.result.blockNumber,
			"contractAddress":receipt_info.result.to,
			"from":receipt_info.result.from,
			"to":receipt_info.result.logs[0].topics[2].clone(),
			"amount":receipt_info.result.logs[0].data.clone(),
			"txid":transaction.hash.clone()
			};
			docs.push(doc.clone());	
	    	}else{
			println!("find a other erc20 transfer");
	   	 }
	}
        // Insert document into 'test.movies' collection
	//println!("----insert--transaction---{}",docs);
	if(docs.len() != 0){
        coll.insert_many(docs, None)
            .ok()
            .expect("Failed to insert document.transaction");
	}


    }
}
/**
impl OpertionMongo for Transaction {
    fn insert(&self) {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

        let coll = client.db("eth").collection("transactions");
	//库里没有u128
	//let amount  = u64::from_str_radix(&self.value.get(2..).unwrap(), 16).unwrap();

        let doc = doc! {
            "blockHash":&self.blockHash,
            "blockNumber":&self.blockNumber,
            "chainId":&self.chainId,
            "condition":&self.condition,
        "creates":&self.creates,
        "from":&self.from,
        "gas":&self.gas,
        "gasPrice":&self.gasPrice,
        "hash":&self.hash,
        "input":&self.input,
        "nonce":&self.nonce,
        "publicKey":&self.publicKey,
        "r":&self.r,
        "raw":&self.raw,
        "s":&self.s,
        "standardV":&self.standardV,
        "to":&self.to,
        "transactionIndex":&self.transactionIndex,
        "v":&self.v,
        "value":&self.value
        };

        // Insert document into 'test.movies' collection
	println!("----insert--transaction---{}",doc);
        coll.insert_one(doc.clone(), None)
            .ok()
            .expect("Failed to insert document.transaction");
    }
}*/
//curl --data '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0xc94770007dda54cF92009BFF0dE90c06F603a09f", "latest"],"id":1}' -H "Content-Type: application/json" -X POST http://119.23.215.121:29842
#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct Address {
    address: String,
}

#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct AloneResult {
    result: String,
    jsonrpc: String,
    id: u32,
}

#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct EthTransaction{
    txid: String,
    amount: String
//后期加上fixme
//    date: String,
}



#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct MistTokenStatus {
    balance: String,
    recent_tx: Vec<EthTransaction>,
}

fn formart_amount(num:&str) ->String {
	let amount = u128::from_str_radix(num.to_string().get(2..).unwrap(), 16).unwrap().to_string();
	let ratio = 10u128.pow(18).to_string();
	let decimal_amount  = Decimal::from_str(&amount).unwrap();
        let decimal_ratio  = Decimal::from_str(&ratio).unwrap();
        let result = decimal_amount / decimal_ratio;
	result.to_string()
}

fn get_eth_txids(address: &str) -> String {
    let data = format! {"{{\"method\":\"eth_getBalance\",\"params\":[\"{}\",\"latest\"],\"id\":1,\"jsonrpc\":\"2.0\"}}",address};
    let list_dir = Command::new("curl")
        .args(&[
            "--data",
            &data,
            "-H",
            "Content-Type: application/json",
            "-X",
            "POST",
            "http://172.18.185.144:29842",
        ])
        .output()
        .expect("ls command failed to start");
    println!("88888-2{:?}", list_dir);
    let info = string_info(list_dir.stdout);
    println!("88888-2{:?}", info);
    let decoded: AloneResult = json::decode(&info).unwrap();
    println!("88888-2{:?}", decoded);



   //let client =
  //          Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

    let coll = CLIENTDB.lock().unwrap().db("eth").collection("transactions");
    let doc = doc! {
        "to": address,
    };  
    let mut opts = FindOptions::new();
    opts.sort = Some(doc! { "blockNumber": -1 }); 
    opts.limit = Some(10); 

    let mut cursor = coll.find(Some(doc.clone()), Some(opts)).expect(
        "Failed to execute find command.",
    );

    println!("{:?}",cursor);
    let mut transactions:Vec<EthTransaction> = Vec::new();
    for result in cursor {
        if let Ok(item) = result {
	    let mut transaction:EthTransaction = Default::default();
            if let Some(&Bson::String(ref hash)) = item.get("hash") {
            	transaction.txid = hash.to_string();
            }   

	    if let Some(&Bson::String(ref value)) = item.get("value") {
	      //transaction.amount = (u128::from_str_radix(&value.to_string().get(2..).unwrap(), 16).unwrap() / 10u128.pow(18)) as f32;
		transaction.amount  = formart_amount(value);
	      println!("transaction.amount ---{}",transaction.amount);
            } 
	    transactions.push(transaction);
        }   
    }  
   // let balance = (u128::from_str_radix(&decoded.result.to_string().get(2..).unwrap(), 16).unwrap() / 10u128.pow(18)) as f32;
	let balance = formart_amount(&decoded.result);
    let mist_usdt_status = MistTokenStatus {
        balance: balance.to_string(),
        recent_tx: transactions,
    };
    json::encode(&mist_usdt_status).unwrap()
}

fn get_usdt_txids(address: &str) -> String {
/*
curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_call","params":[{"to": "0x2482a9c2573b13f70413030004f76b1421749d44", "data":"0x70a08231000000000000000000000000b1648746DabFc8e8c920f57f8b445BC08d3E6675"}, "latest"],"id":67}' http://119.23.215.121:29842
*/
//let data = format! {"{{\"method\":\"eth_getBalance\",\"params\":[\"{}\",\"latest\"],\"id\":1,\"jsonrpc\":\"2.0\"}}",address};
  let data = format! {"{{\"method\":\"eth_call\",\"params\":[{{\"to\":\"0x2482a9c2573b13f70413030004f76b1421749d44\",\"data\":\"0x70a08231000000000000000000000000{}\"}},\"latest\"],\"id\":1,\"jsonrpc\":\"2.0\"}}",address.get(2..).unwrap()};

    let list_dir = Command::new("curl")
        .args(&[
            "--data",
            &data,
            "-H",
            "Content-Type: application/json",
            "-X",
            "POST",
            "http://172.18.185.144:29842",
        ])
        .output()
        .expect("ls command failed to start");
    let info = string_info(list_dir.stdout);
    let decoded: AloneResult = json::decode(&info).unwrap();
//0x000000000000000000000000b1648746dabfc8e8c920f57f8b445bc08d3e6675
    let reciver = format!{"0x000000000000000000000000{}",address.to_string().get(2..).unwrap()};
    let coll = CLIENTDB.lock().unwrap().db("eth").collection("erc20");
    let doc = doc! {
        "to": reciver,
    };  
    let mut opts = FindOptions::new();
    opts.sort = Some(doc! { "blockNumber": -1 }); 
    opts.limit = Some(10); 

    let mut cursor = coll.find(Some(doc.clone()), Some(opts)).expect(
        "Failed to execute find command.",
    );

    println!("{:?}",cursor);
    let mut transactions:Vec<EthTransaction> = Vec::new();
    for result in cursor {
        if let Ok(item) = result {
	    let mut transaction:EthTransaction = Default::default();
            if let Some(&Bson::String(ref txid)) = item.get("txid") {
            	transaction.txid = txid.to_string();
            }   

	    if let Some(&Bson::String(ref amount)) = item.get("amount") {
	      //transaction.amount = (u128::from_str_radix(&value.to_string().get(2..).unwrap(), 16).unwrap() / 10u128.pow(18)) as f32;
		transaction.amount  = formart_amount(amount);
	      println!("transaction.amount ---{}",transaction.amount);
            } 
	    transactions.push(transaction);
        }   
    }  
   // let balance = (u128::from_str_radix(&decoded.result.to_string().get(2..).unwrap(), 16).unwrap() / 10u128.pow(18)) as f32;
	let balance = formart_amount(&decoded.result);
    let mist_eth_status = MistTokenStatus {
        balance: balance.to_string(),
        recent_tx: transactions,
    };
    json::encode(&mist_eth_status).unwrap()
}


fn rpc_service(){
    let mut io = IoHandler::default();

    io.add_method("say_hello", |_| Ok(Value::String("hellossss".into())));

    io.add_method("get_eth_txids", |_params: Params| {
        let address: Address = _params.parse().unwrap();
	let lower_address = &address.address.to_lowercase();
	println!("111122223333--{}",lower_address);
        let result = get_eth_txids(&lower_address);
        Ok(Value::String(result.into()))
    });

    io.add_method("get_usdt_txids", |_params: Params| {
        let address: Address = _params.parse().unwrap();
	let lower_address = &address.address.to_lowercase();
	println!("111122223333--{}",lower_address);
        let result = get_usdt_txids(&lower_address);
        Ok(Value::String(result.into()))
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .start_http(&"0.0.0.0:8030".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
//curl --data '{"method":"eth_getBlockByNumber","params":["0x1b4", true],"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST http://172.18.185.144:29842
//只扫确认的块
//curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'  http://119.23.215.121:29842
fn scan_block() {
   let i =3;
   //let client = Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");
   let coll = CLIENTDB.lock().unwrap().db("eth").collection("block");
   
    let mut opts = FindOptions::new();
    opts.sort = Some(doc! { "blockheight": -1 });
    opts.limit = Some(1);

    let mut cursor = coll.find(None, Some(opts)).expect(
        "Failed to execute find command.",
    );

    println!("{:?}",cursor);
    let mut mongo_height = "".to_string();
    for result in cursor {
        if let Ok(item) = result {
            let mut transaction:EthTransaction = Default::default();
            if let Some(&Bson::String(ref blockheight)) = item.get("blockheight") {
		println!("blockheight{}333",blockheight);
                mongo_height = blockheight.to_string();
            }
        }
    }

    println!("height22222--{}",mongo_height); 

    if  mongo_height == "".to_string() {

    	mongo_height = "0x66d941".to_string();
    }

//    let mut init_height:u64 = mongo_height.parse().unwrap();
    let mut init_height = u64::from_str_radix(&mongo_height.get(2..).unwrap(), 16).unwrap();
    println!("sinit_height----{}",init_height);
    
    loop {
	
	    	let get_height = Command::new("curl")
		   .args(&["--data","{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}","-H",
			"Content-Type: application/json","-X","POST","http://172.18.185.144:29842"])
		 .output()
		 .expect("ls command failed to start");


	
  		let info = string_info(get_height.stdout);	
			    println!("4444-2{:?}", info);
		 let decoded: AloneResult = json::decode(&info).unwrap();
			    println!("111-2{:?}", decoded);
		
		let current_height = u64::from_str_radix(&decoded.result.get(2..).unwrap(), 16).unwrap();
		println!("current height{}",current_height);

		//测试4个上生产12个
		   if current_height - init_height > 4 {
			    let str_height = format!("0x{:x}",init_height + 1);
			    let data = format!("{{\"method\":\"eth_getBlockByNumber\",\"params\":[\"{}\", true],\"id\":1,\"jsonrpc\":\"2.0\"}}",str_height);
			    println!("data={},----{}",data,current_height);
			    let list_dir = Command::new("curl")
				   .args(&["--data",&data,"-H",
					"Content-Type: application/json","-X","POST","http://172.18.185.144:29842"])
				 .output()
				 .expect("ls command failed to start");
			   
			    let info = string_info(list_dir.stdout);
			    let mut decoded: ReturnBlock = json::decode(&info).unwrap();
			   // decoded:number = decoded.number
			    decoded.insert();
			    if decoded.result.transactions.len() != 0{
			   	 decoded.insert_txs();
				decoded.filter_usdt();
			    }
			/*
			    for tx in decoded.result.transactions {
				tx.insert();
				//curl和mongodb都不能太快,eth每秒6个交易，这里每秒20
			    	thread::sleep(time::Duration::from_millis(100));
			    }*/
			    init_height +=1;
			    thread::sleep(time::Duration::from_millis(1000));
		   }else{
			    thread::sleep(time::Duration::from_millis(5000));
	           }
	//	break;
   }
}



fn main() {
    // let mut list_dir = Command::new("/opt/source/bitcoin/src/bitcoin-cli")
    // .args(&["getblock","0000000036b50e0ab347250170b776c1e35156e66ccb9eb0844913e18ef6c363"])
   match fork() {
   Ok(ForkResult::Parent { child, .. }) => {
       println!("Continuing execution in parent process, new child has pid: {}", child);
	rpc_service();
   }
   Ok(ForkResult::Child) => {
	println!("I'm a new child process");
      scan_block();
   }
   Err(_) => println!("Fork failed"),
   }

}
