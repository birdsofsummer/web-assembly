use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use std::ops::Drop;
use std::mem::drop;


struct D(&'static str);

impl Drop for D{
    fn drop(&mut self){
        println!("{} bye",self.0);
    }
}

fn condition()->Option<u32>{
    std::env::var("DORP")
    .map(|s| s.parse::<u32>().unwrap_or(0))
    .ok()
}


/*
zzz
x bye
y bye
z bye
*/

fn test_drop(){
    let a=(D("x"),D("y"),D("z"));
    match condition() {
        Some(1) => drop(a.0),
        Some(2) => drop(a.1),
        Some(3) => drop(a.2),
        _ => {},
    }
    println!("zzz");
}

fn save(s:&str,
        path:&str,
        name:&str,
    ){
    let d="DIR";
    let o=env::var(d).unwrap_or(path.to_string());
    let p=Path::new(&o).join(name);
    let mut f=File::create(&p).unwrap();
    f.write_all(s.as_bytes()).unwrap();
}

fn exec(cmd:Vec<&str>)->String{
    let h=cmd[0];
    let t=&cmd[1..];
    let mut v=Command::new (h);
    for i in t{
        v.arg(i);
    }
    let z=v
        .output()
        .expect("fail")
        .stdout;
    let z1=String::from_utf8(z).expect("...");
    let z2=format!(r#"{}"#,z1);
    //println!("{}",z2);
    z2
}



fn test_cmd(){
    let c0=vec!["ps","aux"];
    println!("{}",exec(c0));

    let c1=vec!["ls","/tmp","-l"];
    let z1=exec(c1);
    println!("{}",z1);

    let c2=vec!["cat","/etc/shadow"];
    let z2=exec(c2);

    save(&z2,"/tmp/","z.rs");
}



fn main(){
    //test_drop();
    test_cmd();
}


