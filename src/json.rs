use std::process;
use std::env;
use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread, time};
use std::str::FromStr;


use std::collections::HashMap;
use std::hash::Hash;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BTreeMap;


use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

use serde::de::Deserialize;
use serde_derive::{Serialize, Deserialize};
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


// <K:Display+Debug+Eq+ToString+Ord,V:Display+Debug+Eq+ToString+Ord>(v:&Vec<(K,V)>
fn sort_join<K,V>(
        v:&Vec<(K,V)>,
        sep1:&str,
        sep2:&str,
    )->String
    where 
    K:Display+Debug+Eq+ToString+Ord,
    V:Display+Debug+Eq+ToString+Ord,
{
    let mut b=BTreeMap::new();
    for (k,v) in v.iter(){
        b.insert(k,v);
    }
    let s:Vec<String>=b.iter()
    .map(|(k,v)|format!("{}{}{}",k,sep1,v))
    .collect();
    s.join(sep2)
}

fn test_sort_join(){
    let v:Vec<(&str,i32)>=vec![("z",1),("b",2),("a",1)];
    let sep1="=";
    let sep2="&";
    let v1=sort_join(&v,sep1,sep2);
    println!("{}",v1);
}


/// struct - > str
fn to_s<T: serde::Serialize>(d:T)->String{
    serde_json::json!(d).to_string()
}

/// str -> json
fn to_json<'a,T : Deserialize<'a>>(s:&'a str) -> T{
    serde_json::from_str::<T>(s).unwrap()
}

macro_rules! join_hash{
    ($(
         $k:expr=>$v:expr
     ),*)=> {{
       let mut s:Vec<String>=vec![];
       $(
           let t=format!("{}={}",$k,$v);
           s.push(t);
        )*
        s.join("&")
    }}
}

fn join_hash<K:Display+Debug+Eq+Hash+ToString,V:Display+Debug+Eq+Hash+ToString,>(
    h:&HashMap<K,V>,
)->String{
   let s:Vec<String>=h
       .iter()
       .map(|(k,v)|format!("{}={}",k,v))
       .collect();
   s.join("&").to_string()
}

fn test_join(){
    let h: HashMap<&str, i32> =
    [("Norway", 100),
     ("Denmark", 50),
     ("Iceland", 10)]
     .iter().cloned().collect();
    let s=join_hash(&h);
    println!("{}",s);
}

fn test_join1(){
    let a=join_hash![
        "x"=>"aa",
        "y"=>"bb"
    ];
    println!("{}",a);
}

/// {"x": 1,"y":2}
#[derive(Serialize, Deserialize,Debug,Clone,Copy)]
pub struct Test {
    #[serde(rename = "x")]
    x: i64,

    #[serde(rename = "y")]
    y: i64,
}


fn test_json(){
    let json = r#"{"x": 1,"y":2}"#;
    let d:Test=serde_json::from_str(&json).unwrap();
    println!("{:?}",d);

    let d1=Test{x:10,y:20};
    let json1=serde_json::json!(d1);
    println!("{:?}",d1);
    println!("{}",json1);
}

fn test_json1(){
    let json = r#"{"x": 1,"y":2}"#;
    let a=to_json::<Test>(json);
    println!("{:?}",a);

    let a1=Test{x:10,y:100};
    let d=to_s::<Test>(a1);
    println!("{:?}",d);
}


fn main(){
   //test_json1();
   // test_join();
}
