extern crate ring;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use ring::{
    digest, rand,
    rand::SecureRandom,
    signature::{self, KeyPair},
};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;

pub mod dealmongo;
#[derive(Deserialize, Debug)]
struct HelloParams {
    fromaccount: String,
    toaccount: String,
    amount: String,
    token: String,
}

#[derive(Deserialize, Debug)]
struct IssueTokenInfo {
    private_key: String,
    account: String,
    token: String,
    amount: f64,
}

#[derive(Debug, Clone)]
pub struct TransferInfo(String, String, String, String, String);

#[derive(Debug, Clone)]
pub struct AccountInfo(String, String, String);

#[derive(Deserialize)]
struct Account {
    account: String,
}

#[derive(Deserialize)]
struct Official {
    official: String,
}



#[derive(Deserialize)]
struct Transaction {
    txid: String,
}

fn analyjson() {
    let mut list_dir = Command::new("ls");
    list_dir.arg("-al");
    let hello = list_dir.status().expect("process failed to execute");
}

/**
A、精度4位、
B、额度上限、100000000000000.0000--一百万亿--精度还没
C、命名规则、只能大写英文字母，长度7位以内（含7位）---还没完成
D、token名称重复的报错--完成
E、用户发行的还是usrccc进行发行，然后走eos转给其对应机构，这个不走mongo的transfer，新建表tokeninfo
F、用户也要传私钥、私钥匹配---完成
其他判断全部交给cleos，shell通过就算通过
**/

fn valid_rule_issue_token(private_key: & str,account : & str,token: & str,amount : & f64) -> bool {
	let mut valid = true;
	let private_key_db = &dealmongo::get_private_key(account);

	println!("private_key={}====account={}==token={}==amount={}==private_key_db=={}",private_key,account,token,amount,private_key_db);
	//这里的浮点型有bug，100000000000000.01显示小于100000000000000.0000,先不管
	if Some(private_key) != Some(private_key_db) 
		|| amount > &100000000000000.0000 
		|| dealmongo::get_token_info(token) {
		valid = false;
	}	
	
	valid
}

fn valid_rule_transfer() -> bool {
	println!("sss");
	let valid = true;
	valid
}
/**

./cleos --url http://23.239.97.98:8888 push action usrccc create '["usrccc", "1000000000.0000 EACD"]' -p usrccc@active;
./cleos --url http://23.239.97.98:8888 push action usrccc issue '[ "usrccc", "1000000000.0000 EACD", "" ]' -p usrccc@active;
**/
pub fn issue_by_eos(){

	let mut list_dir = Command::new("/home/guoxingyun/myproject/exgpc/cleos");
        list_dir.arg("--url");
        list_dir.arg("http://27.155.88.209:8888");
        list_dir.arg("push");
        list_dir.arg("action");
        list_dir.arg("usrbbb");
        list_dir.arg("create");
        list_dir.arg("[\"usrbbb\",\"1000000000.0000 TESTAAB\"]");
        list_dir.arg("-p");
        list_dir.arg("usrbbb@active");
        let getinfo = list_dir.status().expect("process failed to execute");
      //  let getinfo = list_dir.output().expect("process failed to execute");
      //  let mut one = getinfo.stdout;
    //	one.reverse();

      //  let mut all: String = "8888".to_string();
       //  while let Some(top) = one.pop() {
 //           all += &(top as char).to_string();
        // }
//	println!("all={}",all);



}
pub fn registmethod() {
    let mut io = IoHandler::default();

    io.add_method("say_hello", |_| Ok(Value::String("hellossss".into())));

    io.add_method("issue_token", |_params: Params| {
        let parsed: IssueTokenInfo = _params.parse().unwrap();
	
        let mut issue_valid = valid_rule_issue_token(&parsed.private_key,&parsed.account, &parsed.token, &parsed.amount);
	println!("issue_valid={}",issue_valid);
	if issue_valid	== true {

	 crate::dealrpc::issue_by_eos();

	 dealmongo::update_account_info(&parsed.account, &parsed.token, &parsed.amount);
         dealmongo::update_token_info(&parsed.account, &parsed.token, &parsed.amount);

        Ok(Value::String("issue token OK".to_string()))
	}else{
         Ok(Value::String("issue token failed".to_string()))
	}
     });

    io.add_method("account_info", |_params: Params| {
        let parsed: Account = _params.parse().unwrap();

        let mut data = dealmongo::get_account_info(&parsed.account);
        println!("-----------------------{:?}", data);
        let mut return_data = "".to_string();
        while let Some(top) = data.pop() {
            let line = format!("{:?};", top);
            return_data += &line;
        }
        Ok(Value::String(return_data))
    });

    io.add_method("get_transaction", |_params: Params| {
        let parsed: Transaction = _params.parse().unwrap();
        let mut data = dealmongo::get_transaction_info(&parsed.txid);
        println!("-----------------------{:?}", data);
        let mut return_data = "".to_string();
        if let Some(top) = data.pop() {
            return_data = format!("{:?};", top);
        }
        Ok(Value::String(return_data))
    });

    io.add_method("get_info", |_| {
        let mut list_dir = Command::new("curl");
        list_dir.arg("--url");
        list_dir.arg("http://27.155.88.209:8888/v1/chain/get_info");
        let getinfo = list_dir.output().expect("process failed to execute");
        let mut one = getinfo.stdout;
        one.reverse();

        let mut all: String = "".to_string();
        while let Some(top) = one.pop() {
            all += &(top as char).to_string();
        }
        Ok(Value::String(all))
    });

    io.add_method("account_history", |_params: Params| {
        let parsed: Account = _params.parse().unwrap();
        let mut data = dealmongo::account_history(&parsed.account);
        let mut return_data = "".to_string();
        while let Some(top) = data.pop() {
            let line = format!("{:?};", top);
            //	let line = format! ("{"{}","{}","{}","{}","{}"}",top.0,top.1,top.2,top.3,top.4);
            return_data += &line;
        }

        Ok(Value::String(return_data))
    });

    io.add_method("transfer", |_params: Params| {
	 	let parsed: HelloParams = _params.parse().unwrap();
		let data = valid_rule_transfer();
			
		let start = SystemTime::now();
	   	let since_the_epoch = start
		.duration_since(UNIX_EPOCH)
		.expect("Time went backwards");
	    	let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
		let timeAndInfo = b"ms.to_string() + &parsed.fromaccount + &parsed.toaccount + &parsed.amount + &parsed.token";

		let rng = rand::SystemRandom::new();
		let mut buf = vec![0; 96];
		assert!(rng.fill(&mut buf).is_ok());

		let sss = b"sss";
		println!("rng={:?},ms={:?}",buf,sss);
		println!("rng={:?},ms={:?}",buf,&timeAndInfo[..]);
		buf.extend(timeAndInfo.iter().cloned());
		println!("rng={:?}",&buf[..]);
		let buf256 = digest::digest(&digest::SHA256,&buf);
		let selic256 = buf256.as_ref();	
		let mut txid="".to_string();
		let mut  i=0;
		while i < 32 {
			let tmp = format!("{:X}",selic256[i]);
                         txid += &tmp;

			i +=1;
		}
		println!("txid={},",txid);
		
			
		
		
		
		dealmongo::transferinsert(&txid,&parsed.fromaccount,&parsed.toaccount,&parsed.amount,&parsed.token);
                Ok(Value::String(txid))
        });

    io.add_method("create_key", |_params: Params| {


	let parsed: Official = _params.parse().unwrap();

        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let peer_private_key_bytes = pkcs8_bytes.as_ref();

        // Normally the application would store the PKCS#8 file persistently. Later
        // it would read the PKCS#8 file from persistent storage to use it.

        let key_pair =
            signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(pkcs8_bytes.as_ref()))
                .unwrap();

        // Sign the message "hello, world".
        const MESSAGE: &[u8] = b"hello, world";
        let sig = key_pair.sign(MESSAGE);

        // Normally an application would extract the bytes of the signature and
        // send them in a protocol message to the peer(s). Here we just get the
        // public key key directly from the key pair.
        let peer_public_key_bytes = key_pair.public_key().as_ref();
        let sig_bytes = sig.as_ref();

        // Verify the signature of the message using the public key. Normally the
        // verifier of the message would parse the inputs to `signature::verify`
        // out of the protocol message(s) sent by the signer.
        let peer_public_key = untrusted::Input::from(peer_public_key_bytes);
        let msg = untrusted::Input::from(MESSAGE);
        let sig = untrusted::Input::from(sig_bytes);

        println!("public={:?}", peer_public_key_bytes);

        println!("private--sacalr={:?}", peer_private_key_bytes);
        let mut i = 0;
        let m = 0;
        let mut publish_key = "".to_string();
        let mut private_key = "".to_string();

        while i < peer_public_key_bytes.len() {
            let tmp = format!("{:X}", peer_public_key_bytes[i]);
            publish_key += &tmp;
            i += 1;
        }
        while i < peer_private_key_bytes.len() {
            let tmp = format!("{:X}", peer_private_key_bytes[i]);
            private_key += &tmp;
            i += 1;
        }
	let pubkey = publish_key.clone();
	let ptr = publish_key.as_ptr();
	let len = publish_key.len();
	let capacity = publish_key.capacity();
	mem::forget(publish_key);
	let publish_key8 = unsafe { 
		String::from_raw_parts(ptr as *mut _, 8, capacity)
	 };
	
	let address = format!("{}@{}",publish_key8,parsed.official);
        let keypairs = format!("address={},private={}", address, private_key);

        signature::verify(&signature::ED25519, peer_public_key, msg, sig).unwrap();
	
	dealmongo::update_key_info(&private_key,&pubkey,&address);
        Ok(Value::String(keypairs))
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"128.14.178.14:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
