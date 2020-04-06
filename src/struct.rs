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

use std::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

use serde::de::Deserialize;
use serde_derive::{Serialize, Deserialize};
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::time::{Duration, SystemTime,UNIX_EPOCH};
use std::thread::sleep;

use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;


trait S {
    fn new(a:i64,b:i64)->Self;
    fn to_s(&self)->String;
    fn from_s(&mut self,s:&str);
    fn to_qs(&self)->String;
}

/// {"x": 1,"y":2}
#[derive(Serialize, Deserialize,Debug,Clone,Copy)]
pub struct Test {
    #[serde(rename = "x")]
    x: i64,

    #[serde(rename = "y")]
    y: i64,
}


impl S for Test{
    fn new(a:i64,b:i64)->Self{
        Test{
            x:a,
            y:b,
        }
    }
    fn to_s(&self)->String{
        serde_json::json!(&self).to_string()
    }
    fn from_s(&mut self,s:&str){
        *self=serde_json::from_str(&s).unwrap();
    }
    fn to_qs(&self)->String{
        format!("x={}&y={}",self.x,self.y)
    }
}



fn test_s(){
    let a=Test::new(1,2);
    let a1=a.to_s();
    println!("{}",a1);

    let mut b=Test{x:1,y:2};
    let json = r#"{"x": 11,"y":12}"#;
    b.from_s(json);
    println!("{:?}",b);
    b.x=111;

    let y=b.to_qs();
    println!("{}",y);

    /// 解构
    let Test{x:px,y:py} =b;
    println!("px {} py {}",px,py);

}


#[derive(Debug,Clone,Copy)]
struct Color(i32,i32,i32);


fn test_s1(){
    type I = i32;
    let a:I=12;
    
    type T1=Test;
    let b=T1::new(1,200);
    println!("{:?}",b);

    let c=Color(1,2,3);
    println!("{:?}",c);
    println!("{} {} {}",c.0,c.1,c.2);
}


#[derive(Debug,Clone,Copy,)]
enum Code{
    a1=401,
    a2=403,
    a3=404,
    a4,
    a5,
    a6,
}
fn test_e(){
    let code=Code::a5;
    match code{
        Code::a1=>{println!("111{:?}",code);}
        Code::a2=>{println!("222{:?}",code);}
        Code::a3=>{println!("333{:?}",code);}
        _=>{println!("!!!!!{:?}",code);}
    }
}



trait Foo {
    fn echo(&self);
    fn double(&mut self);
}

impl Foo for i32 {
    fn echo(&self){println!("{}",self);}
    fn double(&mut self){*self*=2;}
}

impl Foo for i64 {
    fn echo(&self){println!("{}",self);}
    fn double(&mut self){*self*=2;}
}

/// 高阶函数f=a->b->a*b
fn multiply(m:i32)->Box<dyn Fn(i32)->i32> {
    Box::new(move |x|x*m)
}

fn test1(){
    let a=1_i32;
    let mut b=10_i64;
    a.echo();
    b.echo();
    b.double();
    b.echo();
}

fn test2(){
    let a=10;
    let b=100;
    let m=multiply(a);
    let d=m(b);
    println!("{}*{}={}",a,b,d);
}

fn test3(){
   let a=[1,2,3];
   let b:Vec<i32>=a.iter()
       .map(|&x|x*2)
       .collect();
       //.collect::<Vec<i32>>::();
   println!("{:?}",b);
}



fn main(){
   //test1();
    test_e();

}



