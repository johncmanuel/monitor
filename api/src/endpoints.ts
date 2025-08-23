import { Data } from "../types/data.d.ts";
import { getTrackerData, storeTrackerData } from "../src/kv.ts";

type EndpointHandler = (request: Request) => Promise<Response>;

export const handler: EndpointHandler = async (request) => {
  if (request.method === "POST") {
    if (
      request.headers.get("Authorization") !==
        `Bearer ${Deno.env.get("API_KEY")!}`
    ) {
      console.log("Unauthorized request");
      return new Response("Unauthorized", { status: 401 });
    }
    const data: Data = await request.json();
    console.log(`Received data at timestamp ${data.ts}`);
    await storeTrackerData(data);
    console.log(`successfully stored data for ${data.ts}`);
    return new Response("Data received", { status: 200 });
  } else if (request.method === "GET") {
    console.log("Fetching data");
    const data = await getTrackerData();
    console.log("Fetched data");
    return new Response(JSON.stringify(data), { status: 200 });
  }
  return new Response("Method not allowed", { status: 405 });
};

export const endpoints: Record<string, EndpointHandler> = {
  "/data": handler,
};
