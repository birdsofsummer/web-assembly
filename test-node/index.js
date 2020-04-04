fs = require('fs');

read=async (t)=>{
    let r1=await WebAssembly.compile(t)
    let r2=await WebAssembly.instantiate(r1)
    return r2
}


read1=async (t)=>{
    let typedArray = new Uint8Array(t);
    let env = {
        memoryBase: 0,
        tableBase: 0,
        memory: new WebAssembly.Memory({
          initial: 256
        }),
        table: new WebAssembly.Table({
          initial: 0,
          element: 'anyfunc'
        })
      }
    let r1=await WebAssembly.instantiate(typedArray, {env})
    return r1
}


read2=async (t)=>{
    let r = await WebAssembly.instantiate(t);
    return r
  //  const { instance, module }=r
}

test=async ()=>{
   let file=""
   let t = fs.readFileSync(file);

   let f1=await read2(t)
   let f2=await read(t)
   let f3=await read1(t)

   let r1=f1.instance.exports.add(8, 5)
   let r2=f2.exports.add(12,20)
   let r3=f3.instance.exports.add(1,1)

   console.log(r1,r2,r3)

 ///  let f4=await read1(fs.readFileSync("./c/add.wasm"))
 ///  let r4=f4.instance.exports.add(1,10)


}


test()
