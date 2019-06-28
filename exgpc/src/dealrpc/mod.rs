extern crate ring;
use std::process::Command;
use std::io::{self, Write};
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use std::collections::HashMap;
use serde::Deserialize;
use ring::{
    rand,
    signature::{self, KeyPair},
    rand::SecureRandom,
    digest,
};

use std::time::{SystemTime, UNIX_EPOCH};


pub mod dealmongo;
#[derive(Deserialize)]
#[derive(Debug)]
struct HelloParams {
        fromaccount: String,
	toaccount: String,
	amount: String,
	token: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct TransferInfo (
        String,
	String,
	String,
	String,
	String,
);

#[derive(Deserialize)]
struct HelloParams2 {
        account: String,
}

#[derive(Deserialize)]
struct Transaction {
        txid: String,
}



fn analyjson(){
    let mut list_dir = Command::new("ls");
    list_dir.arg("-al");
    let hello = list_dir.status().expect("process failed to execute");
}

pub fn registmethod(){
	let mut io = IoHandler::default();
	
        io.add_method("say_hello", |_| {
                Ok(Value::String("hellossss".into()))
        });

	io.add_method("account_info", |_params: Params| {
                Ok(Value::String("account_info".into()))
        });

	io.add_method("get_transaction", |_params: Params| {
		let parsed: Transaction = _params.parse().unwrap();
		let mut data = dealmongo::get_transaction_info(&parsed.txid);
		println!("-----------------------{:?}",data);
		let mut return_data = "".to_string();
		if let Some(top) = data.pop() {
			return_data = format! ("{:?};",top);
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

		let mut all:String="".to_string();
		while let Some(top) = one.pop() {
			all += &(top as char).to_string();
		}
 //               Ok(Value::String("sssssssssssssss".into()))
         //      Ok(Value::String(getinfo.stdout.get_mut(1).to_string()))
  	         Ok(Value::String(all))
        });

        io.add_method("account_history", |_params: Params| {
	 let parsed: HelloParams2 = _params.parse().unwrap();
		let mut data = dealmongo::account_history(&parsed.account);
		let mut return_data = "".to_string();
		while let Some(top) = data.pop() {
			let line = format! ("{:?};",top);
		//	let line = format! ("{"{}","{}","{}","{}","{}"}",top.0,top.1,top.2,top.3,top.4);
			return_data += &line;
		}		

                Ok(Value::String(return_data))
        });


	io.add_method("transfer", |_params: Params| {
	 	let parsed: HelloParams = _params.parse().unwrap();
			
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
		
			
		
		
		
		dealmongo::mongoinsert(&txid,&parsed.fromaccount,&parsed.toaccount,&parsed.amount,&parsed.token);
                Ok(Value::String(txid))
        });

	 io.add_method("create_key", |_| {
		//dealmongo::mongoinsert(&parsed.fromaccount,&parsed.toaccount,&parsed.amount,&parsed.token);
		let rng = rand::SystemRandom::new();
		let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
		let peer_private_key_bytes = pkcs8_bytes.as_ref();

		// Normally the application would store the PKCS#8 file persistently. Later
		// it would read the PKCS#8 file from persistent storage to use it.

		let key_pair =
		    signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(pkcs8_bytes.as_ref())).unwrap();

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

		println!("public={:?}",peer_public_key_bytes);

		
		println!("private--sacalr={:?}",peer_private_key_bytes);
		let mut i = 0;
		let m = 0;
		let mut public_key = "".to_string();
		let mut private_key = "".to_string();

		while i < peer_public_key_bytes.len(){
			let tmp = format!("{:X}",peer_public_key_bytes[i]);
                        public_key += &tmp;
			i +=1;

                }
		while i < peer_private_key_bytes.len(){
			let tmp = format!("{:X}",peer_private_key_bytes[i]);
                        private_key  += &tmp;
			i +=1;

                }

		let keypairs = format!("public={},private={}",public_key,private_key);

		
		signature::verify(&signature::ED25519, peer_public_key, msg, sig).unwrap();
                Ok(Value::String(keypairs))
        });


	let server = ServerBuilder::new(io)
                .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
                .start_http(&"128.14.178.14:3030".parse().unwrap())
                .expect("Unable to start RPC server");

        server.wait();
}
