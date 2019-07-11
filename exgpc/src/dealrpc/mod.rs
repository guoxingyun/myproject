extern crate ring;
//#[macro_use]
//extern crate jsonrpc_client_core;
//extern crate jsonrpc_client_http;

use crate::slog::Drain;
use jsonrpc_client_http::HttpTransport;
use std::fs::OpenOptions;

lazy_static! {
    static ref LOGGER: slog::Logger = {
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

jsonrpc_client!(pub struct FizzBuzzClient {
    /// Returns the fizz-buzz string for the given number.
    pub fn fizz_buzz(&mut self, number: u64) -> RpcRequest<String>;
});

use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use ring::{
    digest, rand,
    rand::SecureRandom,
    signature::{self, KeyPair},
};
use serde::Deserialize;

use std::process::Command;

use std::mem;
use std::ops::Mul;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate rust_decimal;
//use std::str::FromStr;
use rust_decimal::Decimal;

use num::ToPrimitive;

pub mod dealmongo;
mod transfer_verify;

#[derive(Deserialize, Debug)]
struct Transfer {
    private_key: String,
    fromaccount: String,
    toaccount: String,
    amount: f64,
    token: String,
}

#[derive(Deserialize, Debug)]
struct SigAndRaw {
    sig: String,
    raw: String,
}

#[derive(Deserialize, Debug)]
struct Sig {
    prikey: String,
    raw: String,
}

#[derive(Deserialize, Debug)]
struct DataInfo {
    head: String, //issue_token,transfer
    fromaccount: String,
    toaccount: String,
    token: String,
    amount: f64,
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
    let _hello = list_dir.status().expect("process failed to execute");
}

fn from_f64(f: f64) -> Option<Decimal> {
    num::FromPrimitive::from_f64(f)
}

pub fn decimal_f64(amount: &f64) -> f64 {
    let mut init_dec = Decimal::new(0, 10);
    let mut amount_new = 0f64;
    if let Some(tmp) = from_f64(*amount) {
        init_dec = tmp;
    };

    if let Some(tmp) = init_dec.to_f64() {
        amount_new = tmp;
    };
    println!("---{}---{}", amount, amount_new);

    amount_new
}

fn valid_amount(amount: &f64) -> bool {
    fn from_f64(f: f64) -> Option<Decimal> {
        num::FromPrimitive::from_f64(f)
    }
    let mut valid = true;

    let myriad: f64 = 10000.0;
    let mut amount_dec = Decimal::new(0, 10);
    let mut myriad_dec = Decimal::new(0, 10);
    let mut amount_mul: f64 = 0.0;

    if let Some(tmp) = from_f64(*amount) {
        amount_dec = tmp;
    };
    if let Some(tmp) = from_f64(myriad) {
        myriad_dec = tmp;
    };

    if let Some(tmp) = amount_dec.mul(myriad_dec).to_f64() {
        amount_mul = tmp;
    };

    if amount_mul != amount_mul.floor() {
        valid = false;
    }

    println!("---------{}---", amount_mul);
    println!("---------{}---", amount.mul(myriad));
    println!("---------{}---", valid);
    valid
}

/**
A、精度4位、
B、额度上限、100000000000000.0000--一百万亿--精度还没
C、命名规则、只能大写英文字母，长度7位以内（含7位）---还没完成
D、token名称重复的报错--完成
E、用户发行的还是usrccc进行发行，然后走eos转给其对应机构，这个不走mongo的transfer，新建表tokeninfo
F、用户也要传私钥、私钥匹配---完成
其他判断全部交给cleos，shell通过就算通过i
除了大小写交给eos处理其他得都在这里判断了
**/

pub fn valid_rule_issue_token(private_key: &str, account: &str, token: &str, amount: &f64) -> bool {
    let mut valid = true;
    let private_key_db = &dealmongo::get_private_key(account);
    let amount_clone = amount.clone();

    println!(
        "private_key={}====account={}==token={}==amount={}==private_key_db=={}",
        private_key, account, token, amount, private_key_db
    );
    //这里的浮点型有bug，100000000000000.01显示小于100000000000000.0000,先不管
    if Some(private_key) != Some(private_key_db)
        || amount_clone > 100000000000000.0000
        || amount_clone < 0.0
        || !valid_amount(amount)
        || dealmongo::get_token_info(token)
        || token.len() > 7
    {
        println!("params is not right");
        valid = false;
    }

    valid
}

/**
     A、交易精度小数点4位，强制4位
        B、交易额超过余额的不能交易而且报错
        C、token不存在的报错-------------------------token重新存一张表中--有人不停的发资产
        D、transfer之后，在account表里做相应的加减
        E、收款地址不存在的，创建记录的时候---因为都是走我们的接口创建，收款地址不存在的就让他正常交易
                私钥存零，表示丢失

        F、发收款的机构和名字都做有效性判断

        G、老王之前的账户的私钥都没有生成处理，让老王自己刷一批私钥和之前的账户绑定。

        H、跨机构走shell填充memo，然后transfer和account都要更新

        I、如何保证用户才能调用转账接口，不能随便一个人都能调用这个接口，---传transfer的时候要传私钥
    J 、自己转自己也不行
**/
fn valid_rule_transfer(
    private_key: &str,
    account_from: &str,
    account_to: &str,
    token: &str,
    amount: &f64,
) -> bool {
    let mut valid = true;
    let private_key_db = &dealmongo::get_private_key(account_from);
    let amount_clone = amount.clone();

    println!(
        "private_key={}====accountfrom={}==accountto={}===token={}==amount={}==private_key_db=={}",
        private_key, account_from, account_to, token, amount, private_key_db
    );

    //待接受的账户也要检查合法性,第九位为@
    let mut aite: String = "".to_string();
    if let Some(tmp) = account_to.to_string().get(8..9) {
        aite = tmp.to_string();
    };

    println!("----{}---ll", aite);
    //这里的浮点型有bug，100000000000000.01显示小于100000000000000.0000,先不管
    if Some(private_key) != Some(private_key_db)
        || amount_clone < 0.0
        || !valid_amount(amount)
        || !dealmongo::get_token_info(token)
        || account_to.to_string() == account_from.to_string()
        || token.len() > 7
        || account_to.len() > 30
        || aite != "@".to_string()
    {
        println!("params is not right in transfer");
        valid = false;
    }

    if amount_clone > dealmongo::get_account_token_balance(&account_from, &token) {
        println!("余额不足");
        valid = false;
    }
    if dealmongo::get_account_token_balance(&account_from, "VSC") < 0.1 {
        println!("手续费不足");
        valid = false;
    }

    valid
}

/**
./cleos --url http://23.239.97.98:8888 push action usrccc create '["usrccc", "1000000000.0000 EACD"]' -p usrccc@active;
./cleos --url http://23.239.97.98:8888 push action usrccc issue '[ "usrccc", "1000000000.0000 EACD", "" ]' -p usrccc@active;
**/
pub fn get_official_from_account(account: &str) -> String {
    let mut account_bytes = account.to_string().into_bytes().to_vec(); //待转给对应机构
    let mut i = 0;
    println!("account_bytes={:?}", account_bytes);
    while i < 9 {
        account_bytes.remove(0);
        i += 1;
    }
    let official = String::from_utf8(account_bytes).unwrap();
    println!("aaaaaaofficial={}", official);
    official
}
pub fn issue_by_eos(account: &str, token: &str, amount: &f64) {
    let official = get_official_from_account(account);

    //    assert!(dealmongo::find_official(&official),"official not exist"); 之前已经通过密钥和账户管理，这里不需要做判断，

    let mut list_dir = Command::new("/home/guoxingyun/myproject/exgpc/cleos");
    list_dir.arg("--url");
    list_dir.arg("http://27.155.88.209:8888");
    list_dir.arg("push");
    list_dir.arg("action");
    list_dir.arg("usrbbb");
    list_dir.arg("create");
    let create_token_amount = format!("[\"usrbbb\",\"{} {}\"]", amount, token);
    //list_dir.arg("[\"usrbbb\",\"1000000000.0000 AAH\"]");

    list_dir.arg(create_token_amount);
    list_dir.arg("-p");
    list_dir.arg("usrbbb@active");
    let getinfo = list_dir.output().expect("process failed to execute");
    let mut one = getinfo.stdout;
    one.reverse();
    let mut create_result: String = "".to_string();
    while let Some(top) = one.pop() {
        create_result += &(top as char).to_string();
    }
    println!("all={}", create_result);
    assert_ne!(create_result, "".to_string(), "create token error");

    let mut list_dir = Command::new("/home/guoxingyun/myproject/exgpc/cleos");
    list_dir.arg("--url");
    list_dir.arg("http://27.155.88.209:8888");
    list_dir.arg("push");
    list_dir.arg("action");
    list_dir.arg("usrbbb");
    list_dir.arg("issue");
    let issue_token_amount = format!("[\"{}\",\"{} {}\",\"\"]", official, amount, token);
    //list_dir.arg("[\"usrbbb\",\"1000000000.0000 AAH\",\"\"]");
    list_dir.arg(issue_token_amount);
    list_dir.arg("-p");
    list_dir.arg("usrbbb@active");
    let getinfo = list_dir.output().expect("process failed to execute");
    let mut one = getinfo.stdout;
    one.reverse();
    let mut issue_result: String = "".to_string();
    while let Some(top) = one.pop() {
        issue_result += &(top as char).to_string();
    }
    println!("all2={}", issue_result);

    assert_ne!(issue_result, "".to_string(), "issue token error");
}

// ../cleos --url http://27.155.88.209:8888  push action usrccc transfer '[ "bdaex", "'${office}'", "'${amount}' '${coin}'", "{\"from\":\"official\",\"to\":\"'${address}'\"}" ]' -p bdaex@active
pub fn transfer_by_eos(account_from: &str, account_to: &str, amount: &f64, token: &str) {
    let official_from = get_official_from_account(account_from);
    let official_to = get_official_from_account(account_to);

    let mut from_prefix = account_from.to_string().clone();
    from_prefix.split_off(8);
    let mut to_prefix = account_to.to_string().clone();
    to_prefix.split_off(8);

    //还要判断token是否为vsc,vsc的具有破坏性最后测试

    let mut list_dir = Command::new("/home/guoxingyun/myproject/exgpc/cleos");
    list_dir.arg("--url");
    list_dir.arg("http://27.155.88.209:8888");
    if token != "VSC".to_string() {
        list_dir.arg("push");
        list_dir.arg("action");
        list_dir.arg("usrbbb");
        list_dir.arg("transfer");
        //这里和老王的json格式少了个大括号，后边改
        let transfer_token_amount = format!(
            "[\"{}\",\"{}\",\"{} {}\",\"\"from\":\"{}\",\"to\":\"{}\"\"]",
            official_from, official_to, amount, token, from_prefix, to_prefix
        );
        println!("transfer_token_amount={}", transfer_token_amount);
        //list_dir.arg("[\"usrbbb\",\"1000000000.0000 AAH\",\"\"]");
        //'[ "bdaex", "'${office}'", "'${amount}' '${coin}'", "{\"from\":\"official\",\"to\":\"'${address}'\"}" ]'

        list_dir.arg(transfer_token_amount);
        list_dir.arg("-p");
        let sigh_official = format!("{}@active", official_from);
        list_dir.arg(sigh_official);
    } else {
        println!("vscccccccccccc");
        //	../cleos --url http://27.155.88.209:8888  transfer bdaex  ${office}  "${amount} VSC" "{\"from\":\"official\",\"to\":\"${address}\"}"
        list_dir.arg("transfer");
        list_dir.arg(official_from);
        list_dir.arg(official_to);
        let amount_vsc = format!("{} VSC", amount);
        println!("ddddd---{}", amount_vsc);
        //这里和老王的json格式少了个大括号，后边改
        list_dir.arg(amount_vsc);
        let transfer_token_amount =
            format!("\"\"from\":\"{}\",\"to\":\"{}\"\"", from_prefix, to_prefix);
        println!("transfer_token_amount={}", transfer_token_amount);
        //  list_dir.arg(transfer_token_amount);
    }

    //let getinfo2 = list_dir.status().expect("process failed to execute");
    let getinfo = list_dir.output().expect("process failed to execute");
    let mut one = getinfo.stdout;
    one.reverse();
    let mut issue_result: String = "".to_string();
    while let Some(top) = one.pop() {
        issue_result += &(top as char).to_string();
    }
    println!("thransfer_return===={}", issue_result);

    assert_ne!(issue_result, "".to_string(), "transfer token error");
}
fn get_height_hash() -> (String, String) {
    let mut list_dir = Command::new("/home/guoxingyun/myproject/exgpc/cleos");
    list_dir.arg("--url");
    list_dir.arg("http://27.155.88.209:8888");
    list_dir.arg("get");
    list_dir.arg("info");
    let getinfo = list_dir.output().expect("process failed to execute");
    let mut one = getinfo.stdout;
    one.reverse();

    let mut all: String = "".to_string();
    while let Some(top) = one.pop() {
        all += &(top as char).to_string();
    }
    println!("info====={}", all);

    //临时先这样写死，后边快高涨到9位数，2年后才出问题
    let mut height = "0".to_string();
    if let Some(tmp) = all.get(136..144) {
        height = tmp.to_string();
    };

    let mut head_hash = "0".to_string();
    if let Some(tmp) = all.get(309..373) {
        head_hash = tmp.to_string();
    };
    (height, head_hash)
}

pub fn registmethod() {
    let mut io = IoHandler::default();

    //let stringkk = num::FromPrimitive::from_f64(2224.0001f64).unwrap().to_string();

    io.add_method("say_hello", |_| {
        info!(LOGGER, "printed {line_count} lines", line_count = 2);
        //transfer_verify::deserialize();

        Ok(Value::String("hellossss".into()))
    });
    //离线签名考虑是发币还是交易做判断

    io.add_method("json_to_bin", |_params: Params| {
        info!(LOGGER, "printed {line_count} lines", line_count = 2);
        let parsed: DataInfo = _params.parse().unwrap();
        let bin = transfer_verify::json_to_bin(
            &parsed.head,
            &parsed.fromaccount,
            &parsed.toaccount,
            &parsed.token,
            &parsed.amount,
        );

        Ok(Value::String(bin))
    });

    io.add_method("sigh_transaction", |_params: Params| {
        info!(LOGGER, "printed {line_count} lines", line_count = 2);

        let parsed: Sig = _params.parse().unwrap();
        let sigdata = transfer_verify::sign_transaction(&parsed.prikey, &parsed.raw);

        Ok(Value::String(sigdata))
    });

    io.add_method("push_transaction", |_params: Params| {
        info!(LOGGER, "printed {line_count} lines", line_count = 2);

        let parsed: SigAndRaw = _params.parse().unwrap();
        let result = transfer_verify::push_transaction(&parsed.sig, &parsed.raw);

        Ok(Value::String(result))
    });

    io.add_method("issue_token", |_params: Params| {
        let parsed: IssueTokenInfo = _params.parse().unwrap();

        let amount = decimal_f64(&parsed.amount);
        let issue_valid =
            valid_rule_issue_token(&parsed.private_key, &parsed.account, &parsed.token, &amount);
        println!("issue_valid={}", issue_valid);
        if issue_valid == true {
            crate::dealrpc::issue_by_eos(&parsed.account, &parsed.token, &amount);

            dealmongo::update_account_info(&parsed.account, &parsed.token, &amount);
            dealmongo::update_token_info(&parsed.account, &parsed.token, &amount);

            Ok(Value::String("issue token OK".to_string()))
        } else {
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
        let (height, hash) = get_height_hash();
        let all = format!("--height={}--hash==={}", height, hash);
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
        let parsed: Transfer = _params.parse().unwrap();

        let amount = decimal_f64(&parsed.amount);

        let valid_transfer = valid_rule_transfer(
            &parsed.private_key,
            &parsed.fromaccount,
            &parsed.toaccount,
            &parsed.token,
            &amount,
        );
        if valid_transfer == false {
            return Ok(Value::String("params is not right".to_string()));
        }
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let _ms = since_the_epoch.as_secs() as i64 * 1000i64
            + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
        //let timeAndInfo = b"ms.to_string() + &parsed.fromaccount + &parsed.toaccount + &amount + &parsed.token"; //偷懒但是仍能保证txid的唯一性
        let timeAndInfo =
            b"ms.to_string() + &parsed.fromaccount + &parsed.toaccount + &parsed.token";

        let rng = rand::SystemRandom::new();
        let mut buf = vec![0; 96];
        assert!(rng.fill(&mut buf).is_ok());

        println!("rng={:?},ms={:?}", buf, &timeAndInfo[..]);
        buf.extend(timeAndInfo.iter().cloned());
        println!("rng={:?}", &buf[..]);
        let buf256 = digest::digest(&digest::SHA256, &buf);
        let selic256 = buf256.as_ref();
        let mut txid = "".to_string();
        let mut i = 0;
        while i < 32 {
            let tmp = format!("{:X}", selic256[i]);
            txid += &tmp;
            i += 1;
        }

        println!("txid={},", txid);

        let new_amount_fromaccount =
            dealmongo::get_account_token_balance(&parsed.fromaccount, &parsed.token) - &amount;

        let new_amount_toaccount =
            &amount + dealmongo::get_account_token_balance(&parsed.toaccount, &parsed.token);

        println!(
            "--{}---{}--{}--",
            dealmongo::get_account_token_balance(&parsed.fromaccount, &parsed.token),
            amount,
            dealmongo::get_account_token_balance(&parsed.toaccount, &parsed.token)
        );

        //机构不同得走eos通道，txid用自己得不用eos的
        if get_official_from_account(&parsed.fromaccount)
            != get_official_from_account(&parsed.toaccount)
        {
            transfer_by_eos(
                &parsed.fromaccount,
                &parsed.toaccount,
                &amount,
                &parsed.token,
            );
        }

        dealmongo::update_account_info(&parsed.fromaccount, &parsed.token, &new_amount_fromaccount);
        dealmongo::update_account_info(&parsed.toaccount, &parsed.token, &new_amount_toaccount);

        //每笔交易扣除0.1的手续费,eos侧数据同也做
        let after_fee_amount_fromaccont =
            dealmongo::get_account_token_balance(&parsed.fromaccount, "VSC") - 0.1;
        let after_fee_amount_toaccount =
            dealmongo::get_account_token_balance("2BCCA62F@gxy111111112", "VSC") + 0.1;

        dealmongo::update_account_info("2BCCA62F@gxy111111112", "VSC", &after_fee_amount_toaccount);
        dealmongo::update_account_info(&parsed.fromaccount, "VSC", &after_fee_amount_fromaccont);
        let fee_eos = 0.1f64;
        transfer_by_eos(
            &parsed.fromaccount,
            "2BCCA62F@gxy111111112",
            &fee_eos,
            "VSC",
        );

        dealmongo::transferinsert(
            &txid,
            &parsed.fromaccount,
            &parsed.toaccount,
            &amount,
            &parsed.token,
        );

        Ok(Value::String(txid))
    });

    io.add_method("create_key", |_params: Params| {
        let parsed: Official = _params.parse().unwrap();

        if dealmongo::find_official(&parsed.official) == false {
            return Ok(Value::String("official not exist".to_string()));
        }

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

        println!("sig={:?}", sig_bytes);
        println!("peer_private_key_bytes={:?}", peer_private_key_bytes);
        println!("peer_public_key_bytes={:?}", peer_public_key_bytes);
        println!("MESSAGE={:?}", MESSAGE);

        let peer_public_key = untrusted::Input::from(peer_public_key_bytes);
        let msg = untrusted::Input::from(MESSAGE);
        let sig = untrusted::Input::from(sig_bytes);

        println!("public={:?}", peer_public_key_bytes);

        println!("private--sacalr={:?}", peer_private_key_bytes);
        let _m = 0;
        let mut publish_key = "".to_string();
        let mut private_key = "".to_string();

        let mut i = 0;
        while i < peer_public_key_bytes.len() {
            let mut tmp = "".to_string();
            if (peer_public_key_bytes[i] < 16) {
                tmp = format!("0{:X}", peer_public_key_bytes[i]);
            } else {
                tmp = format!("{:X}", peer_public_key_bytes[i]);
            }
            publish_key += &tmp;
            i += 1;
        }

        let mut i = 0;
        while i < peer_private_key_bytes.len() {
            let mut tmp = "".to_string();
            if (peer_private_key_bytes[i] < 16) {
                tmp = format!("0{:X}", peer_private_key_bytes[i]);
            } else {
                tmp = format!("{:X}", peer_private_key_bytes[i]);
            }
            private_key += &tmp;
            i += 1;
        }
        let pubkey = publish_key.clone();
        let mut base58_address = "0".to_string();
        if let Ok(tmp) = cryptonote_base58::to_base58(peer_public_key_bytes.to_vec()) {
            println!("sss={}", tmp);
            base58_address = tmp;
        }

        let mut base58_address8 = "0".to_string();
        if let Some(tmp) = base58_address.get(0..8) {
            base58_address8 = tmp.to_string();
        }

        let address = format!("{}@{}", base58_address8, parsed.official);
        let keypairs = format!("address={},private={}", address, private_key);

        signature::verify(&signature::ED25519, peer_public_key, msg, sig).unwrap();

        dealmongo::update_key_info(&private_key, &pubkey, &address);

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
