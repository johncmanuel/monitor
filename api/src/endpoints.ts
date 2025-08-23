import { Data } from "../types/data.d.ts";
import { getTrackerData, storeTrackerData } from "../src/kv.ts";

type EndpointHandler = (request: Request) => Promise<Response>;

export const trackerHandler: EndpointHandler = async (request) => {
  if (request.method === "POST") {
    if (
      request.headers.get("Authorization") !==
        `Bearer ${Deno.env.get("API_KEY")!}`
    ) {
      return new Response("Unauthorized", { status: 401 });
    }
    const data: Data = await request.json();
    await storeTrackerData(data);
    return new Response("Data received", { status: 200 });
  } else if (request.method === "GET") {
    const data = await getTrackerData();
    return new Response(JSON.stringify(data), { status: 200 });
  }
  return new Response("Method not allowed", { status: 405 });
};

export const endpoints: Record<string, EndpointHandler> = {
  "/tracker": trackerHandler,
};
