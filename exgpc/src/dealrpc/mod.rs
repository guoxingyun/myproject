use std::process::Command;
use std::io::{self, Write};
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use std::collections::HashMap;
use serde::Deserialize;

pub mod dealmongo;
#[derive(Deserialize)]
struct HelloParams {
        fromaccount: String,
	toaccount: String,
	amount: String,
	token: String,
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
        io.add_method("transfer", |_params: Params| {
//		let mut scores = HashMap::new();

	 let parsed: HelloParams = _params.parse().unwrap();
         //       Ok(Value::String(format!("hello, {}", parsed.name))
		

                println!("fromaccount={}----{}",&parsed.fromaccount,&parsed.toaccount);
         //       println!("fromaccount={}----{:?}",&String::from("fromaccount"),scores);
		dealmongo::mongoinsert(&parsed.fromaccount,&parsed.toaccount,&parsed.amount,&parsed.token);
                Ok(Value::String("hellossssbyebye".into()))
        });
	let server = ServerBuilder::new(io)
                .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
                .start_http(&"127.0.0.1:3030".parse().unwrap())
                .expect("Unable to start RPC server");

        server.wait();
}
