//用公钥替换地址
use crate::dealrpc::decimal_f64;
use ring::{digest, rand, rand::SecureRandom, signature};
enum Error {
    InvalidSignature,
}
use std::time::{SystemTime, UNIX_EPOCH};

pub fn json_to_bin(
    head: &str,
    fromaccount: &str,
    toaccount: &str,
    token: &str,
    amount: &f64,
) -> String {
    let from_pubkey = super::dealmongo::get_pubkey_by_account(fromaccount);
    let to_pubkey = super::dealmongo::get_pubkey_by_account(toaccount);
    let amountstr = amount.to_string();
    let headhash = super::dealmongo::get_headhash(fromaccount);
    info!(
        crate::LOGGER,
        "json_to_bin-->amountstr----{}\n\n\n", amountstr
    );
    let amountf64: f64 = amountstr.to_string().parse().unwrap();
    info!(crate::LOGGER, "json_to_bin-->amountf64----{}", amountf64);
    let bin = serialize(head, &from_pubkey, &to_pubkey, token, &amountstr, &headhash);
    bin
}

fn bytes_to_string(bytes: &Vec<u8>) -> String {
    let mut bin = "".to_string();
    let mut i = 0;
    while i < bytes.len() {
        let mut tmp = "".to_string();

        if bytes[i] < 16 {
            tmp = format!("0{:X}", bytes[i]);
        } else {
            tmp = format!("{:X}", bytes[i]);
        }
        bin += &tmp;
        i += 1;
    }
    info!(crate::LOGGER, "bytes_to_string---bin={}", bin);

    bin
}
/*
fn string_to_bytes() ->{

}
*/
fn serialize(
    head: &str,
    from_pubkey: &str,
    to_pubkey: &str,
    token: &str,
    amount: &str,
    headhash: &str,
) -> String {
    //防止pubkey尾部为零的时候不好分割
    let splite = [0, 0, 1, 0, 0].to_vec();
    let mut bytes = splite.clone();

    let mut splite_clone = splite.clone();

    let mut head_bytes = head.to_string().into_bytes();

    let mut from_pubkey_bytes = from_pubkey.to_string().into_bytes();
    let mut to_pubkey_bytes = to_pubkey.to_string().into_bytes();
    let mut token_bytes = token.to_string().into_bytes();
    let mut amount_bytes = amount.to_string().into_bytes();

    let mut headhash_bytes = headhash.to_string().into_bytes();
    info!(crate::LOGGER, "serialize-->headhash={}", headhash);

    bytes.append(&mut head_bytes);
    bytes.append(&mut splite_clone);

    let mut splite_clone = splite.clone();
    bytes.append(&mut from_pubkey_bytes);
    bytes.append(&mut splite_clone);

    let mut splite_clone = splite.clone();
    bytes.append(&mut to_pubkey_bytes);
    bytes.append(&mut splite_clone);

    let mut splite_clone = splite.clone();
    bytes.append(&mut token_bytes);
    bytes.append(&mut splite_clone);

    let mut splite_clone = splite.clone();
    bytes.append(&mut amount_bytes);
    bytes.append(&mut splite_clone);

    let mut splite_clone = splite.clone();
    bytes.append(&mut headhash_bytes);
    bytes.append(&mut splite_clone);

    info!(crate::LOGGER, "serialize-->RAWbytes==={:X?}==", bytes);

    let bin = bytes_to_string(&bytes);

    bin
}

//密文
pub fn sign_transaction(prikey: &str, rawdata: &str) -> String {
    let mut pkcs8_bytes: Vec<u8> = Vec::new();

    info!(
        crate::LOGGER,
        "sign_transaction-->prikey={},rawdata={}", prikey, rawdata
    );
    let mut i = 0;
    while None != prikey.get(i..i + 2) {
        let mut tmp2 = "".to_string();

        if let Some(tmp) = prikey.get(i..i + 2) {
            tmp2 = tmp.to_string();
        }

        if let Ok(tmp3) = u8::from_str_radix(&tmp2, 16) {
            pkcs8_bytes.push(tmp3);
        }
        i += 2;
    }

    info!(crate::LOGGER, "sign_transaction-->{:?}", pkcs8_bytes);

    info!(
        crate::LOGGER,
        "sign_transaction-->pkcs8_bytes={:?}==={}==", pkcs8_bytes, rawdata
    );

    let key_pair =
        signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(&pkcs8_bytes)).unwrap();

    let message = rawdata.to_string().into_bytes();
    info!(crate::LOGGER, "sign_transactionmessage={:?}", message);
    let sig_data = key_pair.sign(&message);
    info!(
        crate::LOGGER,
        "sign_transactionsig_data==={:?}",
        sig_data.as_ref()
    );

    let sign_bin = bytes_to_string(&sig_data.as_ref().to_vec());
    sign_bin
}
//字面量转vec<u8>
pub fn deserialize(rawdata: &str) -> Vec<u8> {
    info!(
        crate::LOGGER,
        "deserialize-->deserialize.rawdata={}", rawdata
    );
    let mut bytes: Vec<u8> = Vec::new();
    let mut i = 0;
    while None != rawdata.get(i..i + 2) {
        let mut tmp2 = "".to_string();

        if let Some(tmp) = rawdata.get(i..i + 2) {
            tmp2 = tmp.to_string();
        }

        if let Ok(tmp3) = u8::from_str_radix(&tmp2, 16) {
            bytes.push(tmp3);
        }
        i += 2;
    }
    info!(crate::LOGGER, "deserializekkkkk{:?}", bytes);
    bytes
}

//目前只是给延签的时候获取公钥用，后续合并
fn analy_rawdata(data: &str) -> Vec<u8> {
    let _bytes: Vec<u8> = Vec::new();
    //"41" -> 41
    let mut serialize_data = deserialize(data);
    //41 -> 65 ->  "A"
    info!(
        crate::LOGGER,
        "analy_rawdataanaly_rawdata======{:?}", serialize_data
    );
    let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
    //"A" ->A -> 15

    info!(crate::LOGGER, "analy_rawdataoutstr======{:?}", outstr);

    let bytes = deserialize(&outstr);
    bytes
}
//验证签名
fn verify_sign(sign_data: &str, raw_data: &str) -> Result<(), Error> {
    //考虑hash值以0结尾的情况，这中分割方式就有问题
    let mut v: Vec<&str> = raw_data.split("0000010000").collect();
    v.reverse();
    let mut pubkeystr = "".to_string();
    //只分离出来pubkey的信息
    v.pop();
    v.pop();
    if let Some(tmp) = v.pop() {
        pubkeystr = tmp.to_string();
    }
    info!(crate::LOGGER, "verify_sign-->pubkeystr=={}", pubkeystr); //这是对应16进制的值，并不是真正的pubkey，还要计算出对应的asicc码,该asicc、码是真实公钥的字符串形式

    let peer_public_key_bytes = analy_rawdata(&pubkeystr);

    let message = raw_data.to_string().into_bytes();
    let sig_bytes = deserialize(sign_data);

    info!(
        crate::LOGGER,
        "verify_sign-->peer_public_key_bytes={:?}", peer_public_key_bytes
    );
    info!(
        crate::LOGGER,
        "verify_sign-->message={:?}===sig_bytes={:?}", message, sig_bytes
    );
    let peer_public_key = untrusted::Input::from(&peer_public_key_bytes);

    let msg = untrusted::Input::from(&message);
    let sig = untrusted::Input::from(&sig_bytes);

    signature::verify(&signature::ED25519, peer_public_key, msg, sig)
        .map_err(|_| Error::InvalidSignature)
}

pub fn get_txid(_fromaccount: &str, _toaccount: &str, _token: &str, _amount: &f64) -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let _ms = since_the_epoch.as_secs() as i64 * 1000i64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    //let timeAndInfo = b"ms.to_string() + &parsed.fromaccount + &parsed.toaccount + &amount + &parsed.token"; //偷懒但是仍能保证txid的唯一性
    let timeAndInfo = b"ms.to_string() + fromaccount + toaccount + token";

    let rng = rand::SystemRandom::new();
    let mut buf = vec![0; 96];
    assert!(rng.fill(&mut buf).is_ok());

    info!(
        crate::LOGGER,
        "get_txid-->rng={:?},ms={:?}",
        buf,
        &timeAndInfo[..]
    );
    buf.extend(timeAndInfo.iter().cloned());
    let buf256 = digest::digest(&digest::SHA256, &buf);
    let selic256 = buf256.as_ref();
    let mut txid = "".to_string();
    let mut i = 0;
    while i < 32 {
        let tmp = format!("{:X}", selic256[i]);
        txid += &tmp;
        i += 1;
    }

    info!(crate::LOGGER, "get_txid-->txid={},", txid);
    txid
}

fn getinfo_from_raw(raw_data: &str) -> (String, String, String, String, String, String) {
    let mut v: Vec<&str> = raw_data.split("0000010000").collect();
    v.reverse();
    let mut head = "".to_string();
    let mut from_pubkey = "".to_string();
    let mut to_pubkey = "".to_string();
    let mut token = "".to_string();
    let mut amount = "".to_string();
    let mut headhash = "".to_string();
    v.pop();
    if let Some(tmp) = v.pop() {
        let mut serialize_data = deserialize(tmp);
        let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
        head = outstr.to_string();
    }

    if let Some(tmp) = v.pop() {
        let mut serialize_data = deserialize(tmp);
        let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();

        from_pubkey = outstr.to_string();
    }

    if let Some(tmp) = v.pop() {
        let mut serialize_data = deserialize(tmp);
        let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
        to_pubkey = outstr.to_string();
    }

    if let Some(tmp) = v.pop() {
        info!(crate::LOGGER, "getinfo_from_raw-->tokenname_raw={}", tmp);
        let mut serialize_data = deserialize(tmp);
        let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
        info!(
            crate::LOGGER,
            "getinfo_from_raw-->tokenname_outstr={:?}", outstr
        );
        token = outstr.to_string();
    }

    if let Some(tmp) = v.pop() {
        let mut serialize_data = deserialize(tmp);
        let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
        amount = outstr.to_string();
    }

    if let Some(tmp) = v.pop() {
        let mut serialize_data = deserialize(tmp);
        let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
        headhash = outstr.to_string();
    }

    info!(crate::LOGGER,
        "getinfo_from_raw--->head=={}\n,frompubkey={}\n,to_pubkey={}\n,token={}\n,amount={}\n,headhash={}",
        head, from_pubkey, to_pubkey, token, amount, headhash
    );
    (head, from_pubkey, to_pubkey, token, amount, headhash)
}

fn deal_issuetoken(fromaccount: &str, toaccount: &str, token: &str, amount: &f64) -> String {
    let private_key = super::dealmongo::get_private_key(fromaccount);
    let amount = decimal_f64(amount);
    let issue_valid = super::valid_rule_issue_token(&private_key, fromaccount, token, &amount);
    info!(
        crate::LOGGER,
        "deal_issuetoken--issue_valid={}", issue_valid
    );
    if issue_valid == true {
        crate::dealrpc::issue_by_eos(toaccount, token, &amount);

        super::dealmongo::update_account_info(toaccount, token, &amount);
        super::dealmongo::update_token_info(toaccount, token, &amount);

        "issue token OK".to_string()
    } else {
        "issue token failed".to_string()
    }
}

fn deal_transfer(fromaccount: &str, toaccount: &str, token: &str, amount: &f64) -> String {
    let mut result = "".to_string();

    let amount = decimal_f64(amount);
    let private_key = super::dealmongo::get_private_key(fromaccount);

    //其实之前签名校验的时候已经判断了，这里没必要，但为了复用之前的接口
    let valid_transfer =
        super::valid_rule_transfer(&private_key, fromaccount, toaccount, token, &amount);
    assert!(valid_transfer);

    let from_balance = super::dealmongo::get_account_token_balance(fromaccount, token);
    let new_amount_fromaccount = super::f64_add_sub(&from_balance, &amount, "sub");
    let to_balance = super::dealmongo::get_account_token_balance(toaccount, token);
    let new_amount_toaccount: f64 = super::f64_add_sub(&to_balance, &amount, "add");

    if token == "VSC" && new_amount_fromaccount < 0.1 {
        result = "the rest of vsc is less than fee".to_string();
    } else if new_amount_fromaccount < 0.0 {
        result = "the rest of vsc is less than balance".to_string();
    } else {
    }

    info!(
        crate::LOGGER,
        "deal_transfer-->--{}---{}--{}--",
        super::dealmongo::get_account_token_balance(fromaccount, token),
        amount,
        super::dealmongo::get_account_token_balance(toaccount, token)
    );

    //机构不同得走eos通道，txid用自己得不用eos的
    if super::get_official_from_account(fromaccount) != super::get_official_from_account(toaccount)
    {
        super::transfer_by_eos(fromaccount, toaccount, &amount, token);
    }
    let fee: f64 = 0.1;

    //每笔交易扣除0.1的手续费,eos侧数据同也做
    let after_fee_amount_fromaccount = super::f64_add_sub(
        &super::dealmongo::get_account_token_balance(fromaccount, "VSC"),
        &fee,
        "sub",
    );
    let after_fee_amount_toaccount = super::f64_add_sub(
        &super::dealmongo::get_account_token_balance("2BCCA62F@gxy111111112", "VSC"),
        &fee,
        "add",
    );
    if token == "VSC" {
        let after_fee_amount_fromaccount = super::f64_add_sub(&new_amount_fromaccount, &fee, "sub");
        super::dealmongo::update_account_info(fromaccount, token, &after_fee_amount_fromaccount);
        super::dealmongo::update_account_info(toaccount, token, &new_amount_toaccount);
        super::dealmongo::update_account_info(
            "2BCCA62F@gxy111111112",
            "VSC",
            &after_fee_amount_toaccount,
        );
    } else {
        super::dealmongo::update_account_info(fromaccount, token, &new_amount_fromaccount);
        super::dealmongo::update_account_info(toaccount, token, &new_amount_toaccount);
        super::dealmongo::update_account_info(
            "2BCCA62F@gxy111111112",
            "VSC",
            &after_fee_amount_toaccount,
        );
        super::dealmongo::update_account_info(fromaccount, "VSC", &after_fee_amount_fromaccount);
    }

    let fee_eos = 0.1f64;
    super::transfer_by_eos(fromaccount, "2BCCA62F@gxy111111112", &fee_eos, "VSC");
    result
}

pub fn push_transaction(sign_data: &str, raw_data: &str) -> String {
    match verify_sign(sign_data, raw_data) {
        Ok(_) => println!("verify_sign ok"),
        Err(_err) => {
            return "verify_sign fail".to_string();
        }
    }
    let (head, from_pubkey, to_pubkey, token, amount, headhash) = getinfo_from_raw(raw_data);
    let from_account = super::dealmongo::get_account_by_pubkey(&from_pubkey);
    let current_head_hash = super::dealmongo::get_headhash(&from_account);
    if headhash != current_head_hash {
        return "headhash is diffrent,may be shuanghua".to_string();
    }

    let to_account = super::dealmongo::get_account_by_pubkey(&to_pubkey);
    let amount: f64 = amount.parse().unwrap();
    let headyu = &head;
    let mut result = "".to_string();
    match headyu {
        headyu if headyu == "issuetoken" => {
            result = deal_issuetoken(&from_account, &to_account, &token, &amount);
        }
        headyu if headyu == "transfer" => {
            let result2 = deal_transfer(&from_account, &to_account, &token, &amount);
            if result2 == "".to_string() {
                let txid = get_txid(&from_account, &to_account, &token, &amount);
                let (block_height, block_hash) = super::get_height_hash();

                super::dealmongo::transferinsert(
                    &block_height,
                    &block_hash,
                    &txid,
                    &from_account,
                    &to_account,
                    &amount,
                    &token,
                );
                super::dealmongo::update_headhash(&from_account, &txid);
                result = txid;
            } else {
                result = result2;
            }
        }
        _ => return "head must be transfer or issuetoken".to_string(),
    }
    result
}
