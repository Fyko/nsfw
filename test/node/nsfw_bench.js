import { load } from "nsfwjs";
import { fileURLToPath } from "url";
import { readFile } from "fs/promises";

function bench(start) {
  if (!start) return process.hrtime();
  const stop = process.hrtime(start);
  return (stop[0] * 1e9 + stop[1]) / 1e6;
}

// i wont include loading the module in the benchmark
// because its static in rust
const client = await load();

const start = bench();
const path = fileURLToPath(new URL("../test/puffBounce.gif", import.meta.url));
const file = await readFile(path);
console.log("classifying gif");
const res = await client.classifyGif(file);
console.log("done gif");
const stop = bench(start);

console.dir(res);
console.log(`elapsed: ${stop}ms`);
process.exit(0);
