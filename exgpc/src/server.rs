use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use std::io::{self, Write};
mod dealrpc;

fn main() {
	
//    dealrpc::dealmongo::mongoinsert();
    dealrpc::registmethod();
}
