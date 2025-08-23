// https://github.com/denoland/saaskit/blob/main/tasks/db_dump.ts
import { kv } from "../src/kv.ts";

// https://github.com/GoogleChromeLabs/jsbi/issues/30#issuecomment-521460510
function replacer(_key: unknown, value: unknown) {
  return typeof value === "bigint" ? value.toString() : value;
}

const items = await Array.fromAsync(
  kv.list({ prefix: [] }),
  ({ key, value }) => ({ key, value }),
);
console.log(JSON.stringify(items, replacer, 2));

kv.close();
