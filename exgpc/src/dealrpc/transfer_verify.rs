//用公钥替换地址
use ring::{
    digest, rand,
    rand::SecureRandom,
    signature::{self, KeyPair},
};

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
    println!("b64----{}\n\n\n", amountstr);
    let amountf64: f64 = amountstr.to_string().parse().unwrap();
    println!("b64parse----{}", amountf64);
    let bin = serialize(head, &from_pubkey, &to_pubkey, token, &amountstr,&headhash);
    bin
}

fn bytes_to_string(bytes: &Vec<u8>) -> String {
    let mut bin = "".to_string();
    let mut i = 0;
    while i < bytes.len() {
        let mut tmp = "".to_string();

        if (bytes[i] < 16) {
            tmp = format!("0{:X}", bytes[i]);
        } else {
            tmp = format!("{:X}", bytes[i]);
        }
        bin += &tmp;
        i += 1;
    }
    println!("{}", bin);

    bin
}
/*
fn string_to_bytes() ->{

}
*/
fn serialize(head: &str, from_pubkey: &str, to_pubkey: &str, token: &str, amount: &str,headhash:&str) -> String {
    println!("b64----");
    let mut bytes = [0, 0, 0].to_vec();
    let mut splite = [0, 0, 0].to_vec();
    println!("{}", head);
    let mut head_bytes = head.to_string().into_bytes();

    let mut from_pubkey_bytes = from_pubkey.to_string().into_bytes();
    println!("from_pubkey={}", from_pubkey);
    let mut to_pubkey_bytes = to_pubkey.to_string().into_bytes();
    println!("to_pubkey={}", to_pubkey);
    let mut token_bytes = token.to_string().into_bytes();
    println!("token={}", token);
    let mut amount_bytes = amount.to_string().into_bytes();
    println!("amount={}", amount);

    let mut headhash_bytes = headhash.to_string().into_bytes();
    println!("headhash={}", headhash);



    println!("b64----{:?}", bytes);
    bytes.append(&mut head_bytes);
    bytes.append(&mut splite);

    let mut splite = [0, 0, 0].to_vec();
    println!("b64----{:?}", bytes);
    bytes.append(&mut from_pubkey_bytes);
    bytes.append(&mut splite);

    let mut splite = [0, 0, 0].to_vec();
    println!("b64----{:?}", bytes);
    bytes.append(&mut to_pubkey_bytes);
    bytes.append(&mut splite);
    let mut splite = [0, 0, 0].to_vec();
    println!("b64----{:?}", bytes);
    bytes.append(&mut token_bytes);
    bytes.append(&mut splite);
    let mut splite = [0, 0, 0].to_vec();
    println!("b64----{:?}", bytes);
    bytes.append(&mut amount_bytes);
    bytes.append(&mut splite);
    let mut splite = [0, 0, 0].to_vec();
    bytes.append(&mut headhash_bytes);
    bytes.append(&mut splite);



    println!("RAWbytes==={:X?}==", bytes);

    let bin = bytes_to_string(&bytes);

    bin
}

//密文
pub fn sign_transaction(prikey: &str, rawdata: &str) -> String {
    let mut pkcs8_bytes: Vec<u8> = Vec::new();

    println!("{}", prikey);
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

    println!("{:?}", pkcs8_bytes);

    println!("pkcs8_bytes={:?}==={}==", pkcs8_bytes, rawdata);

    let key_pair =
        signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(&pkcs8_bytes)).unwrap();

    let message = rawdata.to_string().into_bytes();
    println!("message={:?}", message);
    let sig_data = key_pair.sign(&message);
    println!("sig_data==={:?}", sig_data.as_ref());

    let sign_bin = bytes_to_string(&sig_data.as_ref().to_vec());
    sign_bin
}
//字面量转vec<u8>
pub fn deserialize(rawdata: &str) -> Vec<u8> {
    println!("deserialize.rawdata={}", rawdata);
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
    println!("kkkkk{:?}", bytes);
    bytes
}

//目前只是给延签的时候获取公钥用，后续合并
fn analy_rawdata(data: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    //"41" -> 41
    let mut serialize_data = deserialize(data);
    //41 -> 65 ->  "A"
    println!("analy_rawdata======{:?}", serialize_data);
    let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
    //"A" ->A -> 15

    println!("outstr======{:?}", outstr);

    let bytes = deserialize(&outstr);
    bytes
}
//验证签名
fn verify_sign(sign_data: &str, raw_data: &str) -> Result<(), Error> {
    let mut v: Vec<&str> = raw_data.split("000000").collect();
    v.reverse();
    let mut pubkeystr = "".to_string();
    //只分离出来pubkey的信息
    v.pop();
    v.pop();
    if let Some(tmp) = v.pop() {
        pubkeystr = tmp.to_string();
    }
    println!("pubkeystr=={}", pubkeystr); //这是对应16进制的值，并不是真正的pubkey，还要计算出对应的asicc码,该asicc、码是真实公钥的字符串形式

    let peer_public_key_bytes = analy_rawdata(&pubkeystr);

    println!("ss222222222");
    let message = raw_data.to_string().into_bytes();

    println!("ss222222222");
    let sig_bytes = deserialize(sign_data);

    println!("peer_public_key_bytes={:?}", peer_public_key_bytes);
    println!("message={:?}===sig_bytes={:?}", message, sig_bytes);
    let peer_public_key = untrusted::Input::from(&peer_public_key_bytes);

    let msg = untrusted::Input::from(&message);
    println!("ss222222222");
    let sig = untrusted::Input::from(&sig_bytes);

    println!("ss222222222");
    signature::verify(&signature::ED25519, peer_public_key, msg, sig)
        .map_err(|_| Error::InvalidSignature)
}

pub fn get_txid(fromaccount: &str, toaccount: &str, token: &str, amount: &f64) -> String {
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
    txid
}

fn getinfo_from_raw(raw_data: &str) -> (String,String, String, String, String,String) {
    let mut v: Vec<&str> = raw_data.split("000000").collect();
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
        println!("tokenname_raw={}", tmp);
        let mut serialize_data = deserialize(tmp);
        println!("tokenname_serialize_data={:?}", serialize_data);
        let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
        println!("tokenname_outstr={:?}", outstr);
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



    println!(
        "head=={}\n,frompubkey={}\n,to_pubkey={}\n,token={}\n,amount={}\n,headhash={}",
        head, from_pubkey, to_pubkey, token, amount,headhash
    );
    (head,from_pubkey,to_pubkey,token,amount,headhash)
}

fn deal_issuetoken(fromaccount:&str,toaccount:&str,token:&str,amount:&f64) -> String{

	let private_key = super::dealmongo::get_private_key(fromaccount);
	  let amount = super::decimal_f64(amount);
        let issue_valid =
            super::valid_rule_issue_token(&private_key,fromaccount,token, &amount);
        println!("issue_valid={}", issue_valid);
        if issue_valid == true {
            crate::dealrpc::issue_by_eos(toaccount, token, &amount);

            super::dealmongo::update_account_info(toaccount, token, &amount);
            super::dealmongo::update_token_info(toaccount, token, &amount);

          "issue token OK".to_string()
        } else {
           "issue token failed".to_string()
        }

}

fn deal_transfer(fromaccount:&str,toaccount:&str,token:&str,amount:&f64){
        let txid = "0".to_string();

        let amount = super::decimal_f64(amount);
	let private_key = super::dealmongo::get_private_key(fromaccount);
		
	//其实之前签名校验的时候已经判断了，这里没必要，但为了复用之前的接口
            let valid_transfer = super::valid_rule_transfer(   
                &private_key,
                fromaccount,
                toaccount,
                token,
                &amount,
            );
      //      if valid_transfer == false {
                //return Ok(Value::String("params is not right".to_string()));
	//	return
          //  }
	assert!(valid_transfer);



    let new_amount_fromaccount =
                super::dealmongo::get_account_token_balance(fromaccount, token)
                    - &amount;

            let new_amount_toaccount =
                &amount + super::dealmongo::get_account_token_balance(toaccount,token);

            println!(
                "--{}---{}--{}--",
                super::dealmongo::get_account_token_balance(fromaccount, token),
                amount,
                super::dealmongo::get_account_token_balance(toaccount, token)
            );

            //机构不同得走eos通道，txid用自己得不用eos的
            if super::get_official_from_account(fromaccount)
                != super::get_official_from_account(toaccount)
            {
                super::transfer_by_eos(
                    fromaccount,
                    toaccount,
                    &amount,
                    token,
                );
            }

            super::dealmongo::update_account_info(fromaccount, token, &new_amount_fromaccount);
           super::dealmongo::update_account_info(toaccount, token, &new_amount_toaccount);


            //每笔交易扣除0.1的手续费,eos侧数据同也做
            let after_fee_amount_fromaccont = super::dealmongo::get_account_token_balance(fromaccount, "VSC") - 0.1;
            let after_fee_amount_toaccount = super::dealmongo::get_account_token_balance("2BCCA62F@gxy111111112", "VSC") + 0.1;

            super::dealmongo::update_account_info("2BCCA62F@gxy111111112", "VSC", &after_fee_amount_toaccount);
            super::dealmongo::update_account_info(fromaccount, "VSC", &after_fee_amount_fromaccont);
            let fee_eos = 0.1f64;
            super::transfer_by_eos(fromaccount,"2BCCA62F@gxy111111112",&fee_eos,"VSC");


           
}


pub fn push_transaction(sign_data: &str, raw_data: &str) -> String {
    match verify_sign(sign_data, raw_data) {
        Ok(_) => println!("verify_sign ok"),
        Err(err) => {
            println!("verify_sign fail");
            return "verify_sign fail".to_string();
        }
    }
    let (head,from_pubkey,to_pubkey,token,amount,headhash) = getinfo_from_raw(raw_data);
    let from_account = super::dealmongo::get_account_by_pubkey(&from_pubkey);
    let current_head_hash =  super::dealmongo::get_headhash(&from_account);
	if headhash != current_head_hash{
	     return "headhash is diffrent,may be shuanghua".to_string();
	}
	

    let to_account = super::dealmongo::get_account_by_pubkey(&to_pubkey);
    let amount:f64 = amount.parse().unwrap();
   let headyu = &head;
   let mut result  = "".to_string();
    match headyu {
	headyu if headyu == "issuetoken" => {
		result  = deal_issuetoken(&from_account,&to_account,&token,&amount);
	},
	headyu if headyu == "transfer"  => {
		deal_transfer(&from_account,&to_account,&token,&amount);
		let txid = get_txid(&from_account,&to_account,&token,&amount);
		let (block_height,block_hash) = super::get_height_hash();
		
   		super::dealmongo::transferinsert(&block_height,&block_hash,&txid,&from_account,&to_account,&amount,&token);
		super::dealmongo::update_headhash(&from_account,&txid);
		result = txid;
	},
	_ => return "head must be transfer or issuetoken".to_string(),
    }
     result
}

/*
pub fn create_key() -> (String,String){

    let pubkey = "0".to_string();
    let prikey = "0".to_string();
    (pubkey,prikey)
}
*/
