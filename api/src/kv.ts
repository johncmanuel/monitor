import { Data } from "../types/data.d.ts";

export const kv = await Deno.openKv();

export const storeTrackerData = async (data: Data) => {
  const key = ["tracker", String(data.ts)];
  await kv.set(key, data);
};

export const getTrackerData = async () => {
  const data: Data = { ts: BigInt(0), kp: BigInt(0), lc: BigInt(0), rc: BigInt(0), mc: BigInt(0) };
  const entries = kv.list<Data>({ prefix: ["tracker"] });
  for await (const entry of entries) {
    const value = entry.value;
    data.kp += BigInt(value.kp);
    data.lc += BigInt(value.lc);
    data.rc += BigInt(value.rc);
    data.mc += BigInt(value.mc);
  }
  return data;
};

// temporary workaround for bigint issue
export const convertToNumber = (data: Data) => {
  return {
    ts: Number(data.ts),
    kp: Number(data.kp),
    lc: Number(data.lc),
    rc: Number(data.rc),
    mc: Number(data.mc),
  };
}
