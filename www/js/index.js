const R =require("ramda")
const rxjs=require("rxjs")
const superagent=require("superagent")
const moment=require("moment")
const cheerio=require("cheerio")

//import index_html_file from './www/index.html'
const rust = import('../../pkg/fetch1');
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
      let  b=a.add(1,2)

      let u="https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master";
      let  c=await a.get(u+"?x=1&y=2",{x:1,y:2})
      let  d=await a.post(u+"?x=1&y=2",JSON.stringify({dd:123}))
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
