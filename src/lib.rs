//https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RequestInit.html#method.body

extern crate futures;
extern crate serde_derive;
extern crate serde_json;


use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Request, 
    RequestInit, 
    RequestMode, 
    Response,
    ReferrerPolicy,
    Url,
    UrlSearchParams,
};

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::time::{Duration, SystemTime,UNIX_EPOCH};
use std::thread::sleep;

use crate::futures::TryFutureExt;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

impl Signature{
    fn set_name(&mut self,name:String){
        self.name = name;
    }

}



#[derive(Debug, Serialize, Deserialize)]
enum SignatureMethod {
    HmacSHA256, 
    HmacSHA1,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Tsig {
    #[serde(rename = "Action")]
    action: String,

    #[serde(rename = "Region")]
    region: String,

    #[serde(rename = "Timestamp")]
    timestamp: i64,

    #[serde(rename = "Nonce")]
    nonce: i64,

    #[serde(rename = "SecretId")]
    secret_id: String,

    #[serde(rename = "Signature")]
    signature: String,

    #[serde(rename = "Version")]
    version: String,

    #[serde(rename = "SignatureMethod")]
    signature_method: SignatureMethod,

    #[serde(rename = "Token")]
    token: String,
}

/*
pub struct Tsig1{
    Signature : String,
    Tsig
}
*/



fn parse_tsig(json: &str) -> Tsig {
     let model: Tsig = serde_json::from_str(&json).unwrap();
     model
 }


impl Tsig{
    fn new(s:&str) -> Self{
        return parse_tsig(s)
    }
    fn set_token(&mut self,token:String){
        self.token = token;
    }
}



#[wasm_bindgen]
pub async fn get_git() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let u="https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master";
    let request = Request::new_with_str_and_init(u , &opts,).unwrap();

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")
        .unwrap();
    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let resp_value = JsFuture::from(request_promise).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let branch_info: Branch = json.into_serde().unwrap();
    Ok(JsValue::from_serde(&branch_info).unwrap())

}

#[wasm_bindgen]
pub fn get(u:&str) -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.referrer(u);
    opts.referrer_policy(ReferrerPolicy::NoReferrer);
    let request = Request::new_with_str_and_init(&u , &opts,).unwrap();
    request
        .headers()
        .set("Accept", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    request_promise
}
#[wasm_bindgen]
pub fn post(u:&str,body:JsValue) -> Promise {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&body));
    opts.mode(RequestMode::Cors);
    opts.referrer(u);
    opts.referrer_policy(ReferrerPolicy::NoReferrer);
    let request = Request::new_with_str_and_init(&u , &opts,).unwrap();
    request
        .headers()
        .set("Accept", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);
    request_promise
}


#[wasm_bindgen]
pub fn hash()  {
    //https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter
    let mut book_reviews = HashMap::new();
    book_reviews.insert("a".to_string(),"ccc".to_string());
    book_reviews.insert("b".to_string(),"ccc".to_string());
    book_reviews.insert("c".to_string(),"ccc".to_string());
    book_reviews.remove("a");
    let to_find =["b","c"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book)
        }
    }
    println!("b: {}", book_reviews["b"]);
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
        console_log!("{} {}",book, review);
    }
    let l=book_reviews.len();
    console_log!("zzz{}",l);

    let timber_resources: HashMap<&str, i32> =
    [("Norway", 100),
     ("Denmark", 50),
     ("Iceland", 10)]
     .iter().cloned().collect();
    // use the values stored in map
    println!("{:?}",timber_resources);
    console_log!("{:?}",timber_resources);


    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    for (key, val) in map.iter() {
        println!("key: {} val: {}", key, val);
        console_log!("key: {} val: {}", key, val);
    }
    for key in map.keys() {
        console_log!("{}", key);
    }    
    for val in map.values() {
        console_log!("{}", val);
    }    
    for (_, val) in map.iter_mut() {
        *val *= 2;
    }
    for val in map.values_mut() {
        *val = *val + 10;
    }

    for val in map.values() {
        console_log!("{}", val);
    }
}

#[wasm_bindgen]
pub fn add(x:i32,y:i32) -> i32{
    x+y
}

//https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html

#[wasm_bindgen]
pub fn qs(q:JsValue)-> Promise {
    let f=future::ok(q );
    let a=UrlSearchParams::new().unwrap();
    a.append("x","1");
    a.append("y","1");
    //let a=UrlSearchParams::new_with_str_sequence_sequence(&q);
    console_log!("{:?}",a);
    future_to_promise(f)
}

#[wasm_bindgen]
pub fn jv(q:JsValue) -> JsValue {
        let mut s: Signature= q.into_serde().unwrap();
        console_log!("qqqqqqqq {:?}",q);
        console_log!("ssssssss {:?}",s);
        s.set_name("zzz".to_string());
        JsValue::from_serde(&s).unwrap()
}


//https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.UrlSearchParams.html

pub fn now_ms() ->u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis(); //u128
    println!("{}",in_ms);
    in_ms
}

pub fn test_sleep(){
  let now = SystemTime::now();
   sleep(Duration::new(2, 0));
   match now.elapsed() {
       Ok(elapsed) => {
           console_log!("{}", elapsed.as_secs());
       }
       Err(e) => {
           console_log!("Error: {:?}", e);
       }
   }
}


#[wasm_bindgen]
pub fn tsig(q:JsValue) -> JsValue {
        let s: Tsig= q.into_serde().unwrap();
        JsValue::from_serde(&s).unwrap()
}

#[wasm_bindgen]
pub fn tsig1(q:&str) -> JsValue {
        let a=parse_tsig(q);
        console_log!("<<<<{:?}",a);
        JsValue::from_serde(&a).unwrap()
}

#[wasm_bindgen]
pub fn test_tsig1(){
     let q= r#"{"Action":"aa","Region":"bb","Timestamp":0,"Nonce":0,"SecretId":"cc","Signature":"dd","Version":"ee","SignatureMethod":"HmacSHA256","Token":"gg"}"#;
  //   let r=tsig1(q);
     let mut r=Tsig::new(q);
     r.set_token("1234".to_string());
     console_log!("----{:?}",r);
}


#[macro_use]
extern crate stdweb;

pub fn main() {
    stdweb::initialize();
    js! {
        console.log("zzzzzzzz")
    }
    stdweb::event_loop();
}
