import { Data } from "../types/data.d.ts";

export const kv = await Deno.openKv();

export const storeTrackerData = async (data: Data) => {
  const key = ["tracker", String(data.ts)];
  await kv.set(key, data);
};
