//用公钥替换地址
use ring::{
    digest, rand,
    rand::SecureRandom,
    signature::{self, KeyPair},
};
enum Error {
    InvalidSignature,
}
pub fn json_to_bin(head:&str,fromaccount:&str,toaccount:&str,token:&str,amount:&f64) -> String{
	let from_pubkey = super::dealmongo::get_pubkey_by_account(fromaccount);
	let to_pubkey = super::dealmongo::get_pubkey_by_account(toaccount);
	let amount = (123.4567).to_string();
	println!("b64----{}",amount);
	let bin = serialize(head,&from_pubkey,&to_pubkey,token,&amount);
	bin
}

fn bytes_to_string(bytes:&Vec<u8>) -> String{

	let mut bin = "".to_string();
	let mut i = 0;
	while i < bytes.len() {
		
            let mut tmp = "".to_string();

	 if(bytes[i] < 16){
                 tmp = format!("0{:X}", bytes[i]);
         }else{ 
                 tmp = format!("{:X}", bytes[i]);
         }
            bin += &tmp;
            i += 1;
        }
	println!("{}",bin);

	bin

}
/*
fn string_to_bytes() ->{

}
*/
fn serialize(head:&str,from_pubkey:&str,to_pubkey:&str,token:&str,amount:& str) -> String{

	println!("b64----");
	let mut bytes =[0,0,0].to_vec();
	let mut splite = [0,0,0].to_vec();
	println!("{}",head);
	let mut head_bytes = head.to_string().into_bytes();

	let mut from_pubkey_bytes = from_pubkey.to_string().into_bytes();
	println!("from_pubkey={}",from_pubkey);
	let mut to_pubkey_bytes = to_pubkey.to_string().into_bytes();
	println!("to_pubkey={}",to_pubkey);
	let mut token_bytes = token.to_string().into_bytes();
	println!("token={}",token);
	let mut amount_bytes = amount.to_string().into_bytes();
	println!("amount={}",amount);


	println!("b64----{:?}",bytes);
	bytes.append(&mut head_bytes);
		bytes.append(&mut splite);

	let mut splite = [0,0,0].to_vec();
	println!("b64----{:?}",bytes);
		bytes.append(&mut from_pubkey_bytes);
		bytes.append(&mut splite);

	let mut splite = [0,0,0].to_vec();
	println!("b64----{:?}",bytes);
		bytes.append(&mut to_pubkey_bytes);
		bytes.append(&mut splite);
	let mut splite = [0,0,0].to_vec();
	println!("b64----{:?}",bytes);
		bytes.append(&mut token_bytes);
		bytes.append(&mut splite);
	let mut splite = [0,0,0].to_vec();
	println!("b64----{:?}",bytes);
		bytes.append(&mut amount_bytes);
		bytes.append(&mut splite);

	println!("RAWbytes==={:X?}==",bytes);
	

	let bin = bytes_to_string(&bytes); 

	bin

}

//密文
pub fn sign_transaction(prikey:&str,rawdata:&str) -> String{

	let mut pkcs8_bytes:Vec<u8> =Vec::new();

	println!("{}",prikey);
	let mut i = 0;
	while None != prikey.get(i..i+2) {
		
	     let mut tmp2 = "".to_string();
	    
            if let Some(tmp) = prikey.get(i..i+2){
		tmp2 = tmp.to_string();
	   }
	   
	   if let Ok(tmp3) = u8::from_str_radix(&tmp2,16){
		
	   	pkcs8_bytes.push(tmp3);
	   }
            i += 2;
        }

	    println!("{:?}",pkcs8_bytes);

	
	println!("pkcs8_bytes={:?}==={}==",pkcs8_bytes,rawdata);

	 let key_pair =
            signature::Ed25519KeyPair::from_pkcs8(untrusted::Input::from(&pkcs8_bytes))
                .unwrap();

	let message = rawdata.to_string().into_bytes();
println!("message={:?}",message);
        let sig_data = key_pair.sign(&message);
	println!("sig_data==={:?}",sig_data.as_ref());

	let   sign_bin = bytes_to_string(&sig_data.as_ref().to_vec());
	sign_bin
}


pub fn deserialize(rawdata:&str) -> Vec<u8>{


	println!("deserialize.rawdata={}",rawdata);
	let mut bytes:Vec<u8> =Vec::new();
	let mut i = 0;
	while None != rawdata.get(i..i+2) {
		
	     let mut tmp2 = "".to_string();
	    
            if let Some(tmp) = rawdata.get(i..i+2){
		tmp2 = tmp.to_string();
	   }
	   
	   if let Ok(tmp3) = u8::from_str_radix(&tmp2,16){
		
	   	bytes.push(tmp3);
	   }
            i += 2;
        }
	println!("kkkkk{:?}",bytes);
	bytes
      
}

fn analy_rawdata(data:&str) -> Vec<u8>{
	let mut bytes:Vec<u8> =Vec::new();
//"41" -> 41
	let mut serialize_data = deserialize(data);
//41 -> 65 ->  "A"
	println!("analy_rawdata======{:?}",serialize_data);
	let outstr = std::str::from_utf8_mut(&mut serialize_data).unwrap();
//"A" ->A -> 15

	println!("outstr======{:?}",outstr);

	let bytes = deserialize(&outstr);
	bytes
}

fn verify_sign(sign_data:&str,raw_data:&str) -> Result<(), Error>{
	
	let mut v: Vec<&str> = raw_data.split("000000").collect();	  
	v.reverse();
	let mut pubkeystr = "".to_string();
	v.pop();
	v.pop();
	if let Some(tmp) = v.pop(){
		pubkeystr = tmp.to_string();	
	}
	println!("pubkeystr=={}",pubkeystr);//这是对应16进制的值，并不是真正的pubkey，还要计算出对应的asicc码,该asicc、码是真实公钥的字符串形式

	let peer_public_key_bytes = analy_rawdata(&pubkeystr);

	println!("ss222222222");
	let message = raw_data.to_string().into_bytes();

	println!("ss222222222");
	let sig_bytes = deserialize(sign_data);

	println!("peer_public_key_bytes={:?}",peer_public_key_bytes);
	println!("message={:?}===sig_bytes={:?}",message,sig_bytes);	
	let peer_public_key = untrusted::Input::from(&peer_public_key_bytes);	

	  let msg = untrusted::Input::from(&message);
	println!("ss222222222");
          let sig = untrusted::Input::from(&sig_bytes);

	println!("ss222222222");
	signature::verify(&signature::ED25519, peer_public_key, msg, sig).map_err(|_| Error::InvalidSignature)
}


/*
fn get_txid() -> String{

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

}
*/
pub fn push_transaction(sign_data:&str,raw_data:&str) -> String{

	match verify_sign(sign_data,raw_data) {
	  Ok(_) => println!("verify_sign ok"),
	  Err(err) => println!("verify_sign fail"),
	}
/*
	let txid = "0".to_string();

	deserialize(data);
	
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

       

let new_amount_fromaccount =
            dealmongo::get_account_token_balance(&parsed.fromaccount, &parsed.token)
                - &amount;

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
        let after_fee_amount_fromaccont = dealmongo::get_account_token_balance(&parsed.fromaccount, "VSC") - 0.1;
        let after_fee_amount_toaccount = dealmongo::get_account_token_balance("2BCCA62F@gxy111111112", "VSC") + 0.1;

        dealmongo::update_account_info("2BCCA62F@gxy111111112", "VSC", &after_fee_amount_toaccount);
        dealmongo::update_account_info(&parsed.fromaccount, "VSC", &after_fee_amount_fromaccont);
        let fee_eos = 0.1f64;
        transfer_by_eos(&parsed.fromaccount,"2BCCA62F@gxy111111112",&fee_eos,"VSC");


        dealmongo::transferinsert(
            &txid,
            &parsed.fromaccount,
            &parsed.toaccount,
            &amount,
            &parsed.token,
        );


	txid
*/
	"sss".to_string()
}

/*
pub fn create_key() -> (String,String){
	 	
	let pubkey = "0".to_string();
	let prikey = "0".to_string();
	(pubkey,prikey)
}
*/
