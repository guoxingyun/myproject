use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use std::io::{self, Write};
mod dealrpc;

fn main() {
    //    dealrpc::dealmongo::mongoinsert();
    dealrpc::registmethod();
}
