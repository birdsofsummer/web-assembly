const R =require("ramda")
const rxjs=require("rxjs")
const superagent=require("superagent")
const moment=require("moment")
const cheerio=require("cheerio")

//import index_html_file from './www/index.html'
const rust = import('../../pkg/index');
const say=console.log
const {say1}=require("./1.js")


const main=async ()=>{
   say1("now")(moment.now()/1000)
   let u1=location.href+"api?x=1&y=1"
   let sig={name:"ccc",email:"ddd"}
   let s1=JSON.stringify(sig)
   let tc= {
        "Action": "aa",
        "Region": "bb",
        "Timestamp": 0,
        "Nonce": 0,
        "SecretId": "cc",
        "Signature": "dd",
        "Version": "ee",
        "SignatureMethod": "HmacSHA256",
        "Token": "gg"
    }
    let tc1=JSON.stringify(tc)

    try{

      say("R",R)
      let  a=await rust
      console.log(a)
      globalThis.a=a
      let  b=a.add(1,2)
      //a.hi("****")
      let  u="https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master";
      let u1="https://httpbin.org/post";
      let u2="https://httpbin.org/get";

      let  d=await a.post(u1+"?x=212&y=2",JSON.stringify({dd:123}))

      let h= new Headers();
      h.append('C1', 'image/jpeg');
      h.append('C2', 'image/jpeg');
      h.append('C3', 'image/jpeg');
      let  c=await a.get(u2+"?x=11&y=22",h)

      let h1={x:1,y:2,z:3}
      let  c1=await a.get(u2+"?x=11&y=22",h1)

      let  z=await a.get_git()

      say('ccc',c)
      say('ccc',d)
      say('zzz',z)
      say('zzz',z.commit)
      let qr=await a.qs({x:11,y:22})
      say('qqq',qr)
      a.jv(sig)
      a.tsig(tc)
     // a.tsig1(tc1)
      say("add",b);
      say('git',z)
 //       say(c)
 //       say(d)
    }catch(e){
        console.error(e)
    }
}

main()
