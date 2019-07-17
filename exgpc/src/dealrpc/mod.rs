extern crate ring;
//#[macro_use]
//extern crate jsonrpc_client_core;
//extern crate jsonrpc_client_http;

use crate::slog::Drain;

use std::fs::OpenOptions;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use ring::{
    digest, rand,
    rand::SecureRandom,
    signature::{self, KeyPair},
};
use serde::Deserialize;

use std::process::Command;

use std::ops::Mul;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate rust_decimal;
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
struct BlockHash {
    hash: String, //issue_token,transfer
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

#[derive(Deserialize,Debug)]
struct Account {
    account: String,
}

#[derive(Deserialize,Debug)]
struct Official {
    official: String,
}

#[derive(Deserialize,Debug)]
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
    debug!(crate::LOGGER,"decimal_f64-->amount={},amount_new{}", amount, amount_new);

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

    debug!(crate::LOGGER,"valid_amount-->amount_mul={},amount.mul(myriad)={},valid={}", amount_mul,amount.mul(myriad),valid);
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

    info!(crate::LOGGER,
        "valid_rule_issue_token-->private_key={}====account={}==token={}==amount={}==private_key_db=={}",
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
        error!(crate::LOGGER,"params is not right");
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

    info!(crate::LOGGER,
        "valid_rule_transfer-->private_key={},accountfrom={},accountto={},token={},amount={},private_key_db={}",
        private_key, account_from, account_to, token, amount, private_key_db
    );

    //待接受的账户也要检查合法性,第九位为@
    let mut aite: String = "".to_string();
    if let Some(tmp) = account_to.to_string().get(8..9) {
        aite = tmp.to_string();
    };

    debug!(crate::LOGGER,"valid_rule_transfer--->decollator={}", aite);
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
        error!(crate::LOGGER,"params is not right in transfer
	amount_clone={}.amount={},token={}",
	amount_clone,amount,token);

        valid = false;
    }

    if amount_clone > dealmongo::get_account_token_balance(&account_from, &token) {
        error!(crate::LOGGER,"{}'s {} is not enough",account_from,token);
        valid = false;
    }
    if dealmongo::get_account_token_balance(&account_from, "VSC") < 0.1 {
        error!(crate::LOGGER,"VSC fee is not enough,mut be > 0.1");
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
    while i < 9 {
        account_bytes.remove(0);
        i += 1;
    }
    let official = String::from_utf8(account_bytes).unwrap();
    info!(crate::LOGGER,"get_official_from_account-->account={},official={}", account,official);
    official
}
pub fn issue_by_eos(account: &str, token: &str, amount: &f64) {
    let official = get_official_from_account(account);
    //eos发行代币先调用create再调用issue两次完成
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
    list_dir.arg("usrbbb@active"); //合约部署的账户的usrccc测试是usrbbb
    let getinfo = list_dir.output().expect("process failed to execute");
    let mut one = getinfo.stdout;
    one.reverse();
    let mut create_result: String = "".to_string();
    while let Some(top) = one.pop() {
        create_result += &(top as char).to_string();
    }
    debug!(crate::LOGGER,"issue_by_eos-->create_result={}", create_result);
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
    debug!(crate::LOGGER,"issue_by_eos-->issue_result={}", issue_result);

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
        info!(crate::LOGGER,"transfer_by_eos-->notVSC->transfer_token_amount={}", transfer_token_amount);
        //list_dir.arg("[\"usrbbb\",\"1000000000.0000 AAH\",\"\"]");
        //'[ "bdaex", "'${office}'", "'${amount}' '${coin}'", "{\"from\":\"official\",\"to\":\"'${address}'\"}" ]'

        list_dir.arg(transfer_token_amount);
        list_dir.arg("-p");
        let sigh_official = format!("{}@active", official_from);
        list_dir.arg(sigh_official);
    } else {
        info!(crate::LOGGER,"this token is VSC");
        //	../cleos --url http://27.155.88.209:8888  transfer bdaex  ${office}  "${amount} VSC" "{\"from\":\"official\",\"to\":\"${address}\"}"
        list_dir.arg("transfer");
        list_dir.arg(official_from);
        list_dir.arg(official_to);
        let amount_vsc = format!("{} VSC", amount);
        //这里和老王的json格式少了个大括号，后边改
        list_dir.arg(amount_vsc);
        let transfer_token_amount = format!("\"\"from\":\"{}\",\"to\":\"{}\"\"", from_prefix, to_prefix);
        info!(crate::LOGGER,"transfer_by_eos-->transfer_token_amount={}", transfer_token_amount);
    }

    //let getinfo2 = list_dir.status().expect("process failed to execute");
    let getinfo = list_dir.output().expect("process failed to execute");
    let mut one = getinfo.stdout;
    one.reverse();
    let mut issue_result: String = "".to_string();
    while let Some(top) = one.pop() {
        issue_result += &(top as char).to_string();
    }
    info!(crate::LOGGER,"transfer_by_eos-->thransfer_return={}", issue_result);

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
    info!(crate::LOGGER,"get_height_hash-->chaininfo==={}", all);

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

fn get_block_by_eos(hash: &str) -> String {
    let mut list_dir = Command::new("/home/guoxingyun/myproject/exgpc/cleos");
    list_dir.arg("--url");
    list_dir.arg("http://27.155.88.209:8888");
    list_dir.arg("get");
    list_dir.arg("block");
    list_dir.arg(hash);
    let getinfo = list_dir.output().expect("process failed to execute");
    let mut one = getinfo.stdout;
    one.reverse();

    let mut all: String = "".to_string();
    while let Some(top) = one.pop() {
        all += &(top as char).to_string();
    }
    info!(crate::LOGGER,"get_block_by_eos-->blockinfo={}", all);

    all
}

pub fn registmethod() {
    let mut io = IoHandler::default();

    io.add_method("say_hello", |_| {
        info!(crate::LOGGER, "printed {line_count} lines", line_count = 2);
        Ok(Value::String("hellossss".into()))
    });
    //离线签名考虑是发币还是交易做判断

    io.add_method("json_to_bin", |_params: Params| {
        let parsed: DataInfo = _params.parse().unwrap();
	
        info!(crate::LOGGER, "json_to_bin::{:?}",parsed);

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
        let parsed: Sig = _params.parse().unwrap();
        info!(crate::LOGGER, "sigh_transaction::{:?}",parsed);
        let sigdata = transfer_verify::sign_transaction(&parsed.prikey, &parsed.raw);

        Ok(Value::String(sigdata))
    });

    io.add_method("push_transaction", |_params: Params| {
        let parsed: SigAndRaw = _params.parse().unwrap();
        info!(crate::LOGGER, "push_transaction_transaction::{:?}",parsed);
        let result = transfer_verify::push_transaction(&parsed.sig, &parsed.raw);

        Ok(Value::String(result))
    });

    io.add_method("issue_token", |_params: Params| {
        let parsed: IssueTokenInfo = _params.parse().unwrap();
        info!(crate::LOGGER, "issue_token::{:?}",parsed);
        let amount = decimal_f64(&parsed.amount);
        let issue_valid =
            valid_rule_issue_token(&parsed.private_key, &parsed.account, &parsed.token, &amount);
        debug!(crate::LOGGER, "issue_token::issue_valid{}",issue_valid);

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
        info!(crate::LOGGER, "account_info::{:?}",parsed);
        let mut data = dealmongo::get_account_info(&parsed.account);
        let mut return_data = "".to_string();
        while let Some(top) = data.pop() {
            let line = format!("{:?};", top);
            return_data += &line;
        }
        Ok(Value::String(return_data))
    });

    io.add_method("get_transaction", |_params: Params| {
        let parsed: Transaction = _params.parse().unwrap();

        info!(crate::LOGGER, "get_transaction::{:?}",parsed);

        let mut data = dealmongo::get_transaction_info(&parsed.txid);
        info!(crate::LOGGER,"get_transaction-->-ransaction_info=={:?}", data);
        let mut return_data = "".to_string();
        if let Some(top) = data.pop() {
            return_data = format!("{:?};", top);
        }
        Ok(Value::String(return_data))
    });

    //1、getblock的时候mongo有就有返回，没有就去eos拿，拿到就返回高度hash交易为空，拿不到就不存在
    io.add_method("get_block", |_params: Params| {
        let parsed: BlockHash = _params.parse().unwrap();

        info!(crate::LOGGER, "get_block::{:?}",parsed);

        let blockinfo = dealmongo::get_block(&parsed.hash);
        let mut result = "".to_string();
        if blockinfo.len() == 0 {
            error!(crate::LOGGER,"mongo cannt find this hash");
            if get_block_by_eos(&parsed.hash).len() == 0 {
                result = "Invalid block ID".to_string();
            } else {
                result = "[]".to_string();
            }
        } else {
            result = format!("{:?}", blockinfo);
        }

        Ok(Value::String(result))
    });

    io.add_method("get_info", |_| {
        let (height, hash) = get_height_hash();
        let all = format!(
            "vscid:cf057bbfb72640471fd910bcb67639c22d1f,version:V1.0.2,height:{},headhash:{}",
            height, hash
        );
        Ok(Value::String(all))
    });

    io.add_method("account_history", |_params: Params| {
        let parsed: Account = _params.parse().unwrap();

        info!(crate::LOGGER, "account_history::{:?}",parsed);

        let mut data = dealmongo::account_history(&parsed.account);
        let mut return_data = "".to_string();
        while let Some(top) = data.pop() {
            let line = format!("{:?};", top);
            return_data += &line;
        }

        Ok(Value::String(return_data))
    });

    io.add_method("transfer", |_params: Params| {
        let parsed: Transfer = _params.parse().unwrap();

        info!(crate::LOGGER, "Transfer::{:?}",parsed);

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

        info!(crate::LOGGER,"rng={:?},ms={:?}", buf, &timeAndInfo[..]);
        buf.extend(timeAndInfo.iter().cloned());
        let buf256 = digest::digest(&digest::SHA256, &buf);
        let selic256 = buf256.as_ref();
        let mut txid = "".to_string();
        let mut i = 0;
	//取随机值的前32位
        while i < 32 {
            let tmp = format!("{:X}", selic256[i]);
            txid += &tmp;
            i += 1;
        }

        info!(crate::LOGGER,"transfer-->generatetxid={},", txid);

        let new_amount_fromaccount =
            dealmongo::get_account_token_balance(&parsed.fromaccount, &parsed.token) - &amount;

	//防止VSC交易的时候透支额度
	if &parsed.token == "VSC" && new_amount_fromaccount < 0.1 {
		return Ok(Value::String("the rest of vsc is less than fee".to_string()))
	}else if new_amount_fromaccount < 0.0{
		return Ok(Value::String("the rest of vsc is less than balance".to_string()))
	}else{}

        let new_amount_toaccount =
            &amount + dealmongo::get_account_token_balance(&parsed.toaccount, &parsed.token);

        info!(crate::LOGGER,
            "beferotokenbalancefrom={}--amount={}--beforetokenbalanceto{}--afterfrom={}--afterto={}",
            dealmongo::get_account_token_balance(&parsed.fromaccount, &parsed.token),
            amount,
            dealmongo::get_account_token_balance(&parsed.toaccount, &parsed.token),
		new_amount_fromaccount,new_amount_toaccount
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


	//每笔交易扣除0.1的手续费,eos侧数据同也做
	
        let after_fee_amount_fromaccont =
            dealmongo::get_account_token_balance(&parsed.fromaccount, "VSC") - 0.1;
	
        let after_fee_amount_toaccount =
            dealmongo::get_account_token_balance("2BCCA62F@gxy111111112", "VSC") + 0.1;
	let after_fee_amount_fromaccont = decimal_f64(&after_fee_amount_fromaccont);
	let after_fee_amount_toaccount = decimal_f64(&after_fee_amount_toaccount);

	if &parsed.token == "VSC"{
		let after_fee_amount_fromaccont = new_amount_fromaccount - 0.1;
		let after_fee_amount_fromaccont = decimal_f64(&after_fee_amount_fromaccont);
        	dealmongo::update_account_info(&parsed.fromaccount, &parsed.token, &after_fee_amount_fromaccont);
        	dealmongo::update_account_info(&parsed.toaccount, &parsed.token,&new_amount_toaccount);
        	dealmongo::update_account_info("2BCCA62F@gxy111111112", "VSC", &after_fee_amount_toaccount);
	}else{
        	dealmongo::update_account_info(&parsed.fromaccount, &parsed.token, &new_amount_fromaccount);
		dealmongo::update_account_info(&parsed.toaccount, &parsed.token, &new_amount_toaccount);
		dealmongo::update_account_info("2BCCA62F@gxy111111112", "VSC", &after_fee_amount_toaccount);
		dealmongo::update_account_info(&parsed.fromaccount, "VSC", &after_fee_amount_fromaccont);
	}
        let fee_eos = 0.1f64;
        transfer_by_eos(
            &parsed.fromaccount,
            "2BCCA62F@gxy111111112",
            &fee_eos,
            "VSC",
        );

        let (block_height, block_hash) = get_height_hash();
        dealmongo::transferinsert(
            &block_height,
            &block_hash,
            &txid,
            &parsed.fromaccount,
            &parsed.toaccount,
            &amount,
            &parsed.token,
        );

        dealmongo::update_headhash(&parsed.fromaccount, &txid);

        Ok(Value::String(txid))
    });

    io.add_method("create_key", |_params: Params| {
        let parsed: Official = _params.parse().unwrap();
        info!(crate::LOGGER, "create_key::{:?}",parsed);

        if dealmongo::find_official(&parsed.official) == false {
            return Ok(Value::String("official not exist".to_string()));
        }

        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let peer_private_key_bytes = pkcs8_bytes.as_ref();

        let key_pair =
            signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(pkcs8_bytes.as_ref()))
                .unwrap();

        let peer_public_key_bytes = key_pair.public_key().as_ref();

        debug!(crate::LOGGER,"create_key--->peer_private_key_bytes={:?}", peer_private_key_bytes);
        debug!(crate::LOGGER,"create_key--->peer_public_key_bytes={:?}", peer_public_key_bytes);

        let peer_public_key = untrusted::Input::from(peer_public_key_bytes);

        let _m = 0;
        let mut publish_key = "".to_string();
        let mut private_key = "".to_string();

        let mut i = 0;
        while i < peer_public_key_bytes.len() {
            let mut tmp = "".to_string();
            if peer_public_key_bytes[i] < 16 {
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
            if peer_private_key_bytes[i] < 16 {
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
            base58_address = tmp;
        }

        let mut base58_address8 = "0".to_string();
        if let Some(tmp) = base58_address.get(0..8) {
            base58_address8 = tmp.to_string();
        }

        let address = format!("{}@{}", base58_address8, parsed.official);
        let keypairs = format!("address={},private={}", address, private_key);

        //考虑极低概率前八位hash碰撞的情况
        if dealmongo::get_pubkey_by_account(&address).len() != 0 {
            return Ok(Value::String(
                "unknown error! please test again".to_string(),
            ));
        }

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
