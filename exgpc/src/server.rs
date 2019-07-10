mod dealrpc;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

#[macro_use]
extern crate lazy_static;

use slog::Drain;
use std::fs::OpenOptions;
/**
lazy_static!{

   static ref LOGGER = {
   let log_path = "/home/guoxingyun/myproject/exgpc/your_log_file_path.log";
   let file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(log_path)
      .unwrap();

    let decorator = slog_term::PlainDecorator::new(file);
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = slog::Logger::root(drain, o!());
    _log
    };
}
**/
fn main() {
    //    dealrpc::dealmongo::mongoinsert();
    // let logger:'static = loginit();
    dealrpc::registmethod();
}
