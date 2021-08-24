const addon = require('./index.node');
const { promisify } = require("util");
const rayon_pool_task = promisify(addon.rayon_pool_task);
// const wasmFib = require('wasm-fibonacci');

// addon.start_task(function(...args) {
//     console.log(args)
// // });
// const x = Error('I was created using a function call!');
// console.log( x)
// setTimeout(() => {
    
// }, 10000);

// rayon_pool_task(5).then(console.log).catch(err => {
//     console.log(err)
//     console.log(typeof err)
//     console.log(err.message)
//     console.log(err.stack)
// })

// // var start = Date.now();
addon.rayon_pool_task(5, function(err, r) {
    console.log(new Error(err.message), r)
    // console.log(Date.now() - start)
});

// setInterval(() => {
//     console.log("---=-=-=")
// }, 1000);

// console.log(addon.max_num());

// 可粗略计算 eventloop time
// (function im(){
//     const start = hrtime.bigint();
//     setImmediate(() => {
//         const end = hrtime.bigint();
//         console.log(`Benchmark took ${end - start} nanoseconds`);
//         im()
//     });
// })();




// var start = Date.now();
// console.log(wasmFib(40))
// console.log(Date.now() - start)
// console.log("----")
// // let buf = Buffer.from([1,2]);
// // const ab = new ArrayBuffer(10);
// // console.log(ab)
// var start = Date.now();
// console.log(addon.fib(40));
// console.log(Date.now() - start)
// console.log("----")

// var start = Date.now();
// console.log(fib(40));
// console.log(Date.now() - start)

// function fib(i) {
//     if (i == 1) return 1;
//     if (i == 2) return 1;
//     return fib(i-1) + fib(i-2);
// }