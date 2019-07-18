mod dealrpc;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
use std::fs::OpenOptions;
use crate::slog::Drain;

#[macro_use]
extern crate lazy_static;


 use std::time::SystemTime;

lazy_static!{
   static ref LOGGER: slog::Logger  = {
   let log_path = "/home/guoxingyun/myproject/exgpc/your_log_file_path.log";
   let file = OpenOptions::new()
      .create(true)
      .write(true)
      //.truncate(false)
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

use std::time::Instant;
fn main() {

let now = Instant::now();
   println!("iii{:?}",now);
    dealrpc::registmethod();
}
