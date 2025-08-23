import { Data } from "../types/data.d.ts";

export const kv = await Deno.openKv();

export const storeTrackerData = async (data: Data) => {
  const key = ["tracker", String(data.ts)];
  await kv.set(key, data);
};

export const getTrackerData = async () => {
  const data: Data[] = [];
  const entries = kv.list<Data>({ prefix: ["tracker"] });
  for await (const entry of entries) {
    const value = entry.value;
    data.push(value);
  }
  return data;
};
