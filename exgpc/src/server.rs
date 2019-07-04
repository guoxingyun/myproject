mod dealrpc;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;


fn main() {
    //    dealrpc::dealmongo::mongoinsert();
    dealrpc::registmethod();
}
