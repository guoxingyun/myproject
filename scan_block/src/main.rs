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
}

#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct BaseInfo {}
#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
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
#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
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

impl OpertionMongo for ReturnBlock {
    fn insert(&self) {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

        let coll = client.db("eth").collection("block");
        let block_height = self.result.number.clone();
        let block_hash = self.result.hash.clone();
        let txids = format!("{:?}", self.result.transactions);
        let date = self.result.timestamp.clone();

        println!("11111{}--22{:?}", txids, self.result.transactions);
        let doc = doc! {
        "blockheight": &block_height,
        "blockhash": &block_hash,
        "txids": &txids,
        "date": &date,
        };

        // Insert document into 'test.movies' collection
        coll.insert_one(doc.clone(), None)
            .ok()
            .expect("Failed to insert document.");
    }
}

impl OpertionMongo for Transaction {
    fn insert(&self) {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

        let coll = client.db("eth").collection("transactions");

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
        coll.insert_one(doc.clone(), None)
            .ok()
            .expect("Failed to insert document.");
    }
}
//curl --data '{"method":"eth_getBlockByNumber","params":["0x1b4", true],"id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST http://172.18.185.144:29842
fn scan_block() {
    let list_dir = Command::new("curl")
           .args(&["--data","{\"method\":\"eth_getBlockByNumber\",\"params\":[\"0x111b43\", true],\"id\":1,\"jsonrpc\":\"2.0\"}","-H",
		"Content-Type: application/json","-X","POST","http://172.18.185.144:29842"])
         .output()
         .expect("ls command failed to start");
    println!("88888-2{:?}", list_dir);
    let info = string_info(list_dir.stdout);
    println!("88888-2{:?}", info);
    let decoded: ReturnBlock = json::decode(&info).unwrap();
    println!("88888-2{:?}", decoded);
    decoded.insert();
    for tx in decoded.result.transactions {
        tx.insert();
    }
}

//curl --data '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0xc94770007dda54cF92009BFF0dE90c06F603a09f", "latest"],"id":1}' -H "Content-Type: application/json" -X POST http://119.23.215.121:29842
#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct Address {
    address: String,
}

#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct EthBalance {
    result: String,
    jsonrpc: String,
    id: u32,
}

#[derive(RustcDecodable, RustcEncodable, Deserialize, Debug, Default)]
struct MistEthBalance {
    balance: String,
    last_txid: String,
    amount: String
}

fn get_address_info(address: &str) -> String {
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
    let decoded: EthBalance = json::decode(&info).unwrap();
    println!("88888-2{:?}", decoded);



   let client =
            Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");

        let coll = client.db("eth").collection("transactions");

    let doc = doc! {
        "to": address,
    };  
    let mut opts = FindOptions::new();
    opts.sort = Some(doc! { "blockNumber": -1 }); 
    opts.limit = Some(1); 

    let mut cursor = coll.find(Some(doc.clone()), Some(opts)).expect(
        "Failed to execute find command.",
    );

    println!("{:?}",cursor);

    let mut amount = "".to_string();
    let mut txid  = "".to_string();
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref hash)) = item.get("hash") {
                txid = hash.to_string();
            }   

	    if let Some(&Bson::String(ref value)) = item.get("value") {
                amount = value.to_string();
            } 
        }   
    }  

    let mist_etn_balance = MistEthBalance {
        balance: decoded.result,
        last_txid: txid,
	amount: amount
    };
    format!("{:?}", mist_etn_balance)
}

fn main() {
    // let mut list_dir = Command::new("/opt/source/bitcoin/src/bitcoin-cli")
    // .args(&["getblock","0000000036b50e0ab347250170b776c1e35156e66ccb9eb0844913e18ef6c363"])

    let mut io = IoHandler::default();

    io.add_method("say_hello", |_| Ok(Value::String("hellossss".into())));

    io.add_method("get_account_status", |_params: Params| {
        let address: Address = _params.parse().unwrap();
        let result = get_address_info(&address.address);
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
