const fs = require("fs");
const util = require("util");
const path = require("path");
const readFile = util.promisify(fs.readFile);
const { PerformanceObserver, performance } = require("perf_hooks");
const addon = require("./index.node");

async function main() {
  let num = 55;
  const buf = await readFile(path.resolve(__dirname, "./fib.wasm"));

  const res = await WebAssembly.instantiate(new Uint8Array(buf.buffer));

  const { fib: wasmFib } = res.instance.exports;

  var start = Date.now();
  console.log("wasm", wasmFib(num));
  console.log('time: ',Date.now() - start);
  console.log("----");
  // let buf = Buffer.from([1,2]);
  // const ab = new ArrayBuffer(10);
  // console.log(ab)
  var start = Date.now();
  console.log("addon", addon.fib(num));
  console.log('time: ',Date.now() - start);
  console.log("----");

  // var start = Date.now();
  // console.log('js:', fibonacci(num));
  // console.log('time: ',Date.now() - start);
  // var start = Date.now();
  // console.log("---");
  // setInterval(() => {
  // }, 1000);
  addon.rayon_pool_task(num, function (r) {
    console.log("rayon", r);
    console.log('time: ',Date.now() - start);
  });

  //   bench('WASM', fib)
  //   bench('JS', fibonacci)
}

function bench(label, fn) {
  const N_ITERS = 30;

  const observations = [];
  const obs = new PerformanceObserver((list) => {
    const entries = list.getEntries();
    observations.push(...entries);
    performance.clearMarks();
  });

  obs.observe({ entryTypes: ["measure"] });

  for (let i = 0; i <= N_ITERS; ++i) {
    performance.mark("A");
    fn(40);
    performance.mark("B");
    performance.measure("A to B", "A", "B");
  }

  const total = observations.reduce((total, e) => total + e.duration, 0);
  const avg = total / observations.length;
  const totalFx = Number.parseFloat(total).toFixed(2);
  const avgFx = Number.parseFloat(avg).toFixed(2);
  console.log(`[${label}] total: ${totalFx} ms average: ${avgFx} ms`);
}

function fibonacci(num) {
  if (num <= 1) return 1;

  return fibonacci(num - 1) + fibonacci(num - 2);
}

main();
