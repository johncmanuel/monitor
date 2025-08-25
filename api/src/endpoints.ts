import { Data } from "../types/data.d.ts";
import {
  convertToNumber,
  getTrackerData,
  storeTrackerData,
} from "../src/kv.ts";

type EndpointHandler = (request: Request) => Promise<Response>;

const allowedOrigins = ["https://johncarlomanuel.com"];

const corsHeaders = {
  "Access-Control-Allow-Origin": allowedOrigins.join(", "),
  "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type, Authorization",
};

export const handler: EndpointHandler = async (request) => {
  if (request.method === "OPTIONS") {
    return new Response(null, { status: 204, headers: corsHeaders });
  }

  if (request.method === "POST") {
    if (
      request.headers.get("Authorization") !==
      `Bearer ${Deno.env.get("API_KEY")!}`
    ) {
      console.log("Unauthorized request");
      return new Response("Unauthorized", { status: 401, headers: corsHeaders });
    }
    const data: Data = await request.json();
    console.log(`Received data at timestamp ${data.ts}`);
    await storeTrackerData(data);
    console.log(`successfully stored data for ${data.ts}`);
    return new Response("Data received", { status: 200, headers: corsHeaders });
  } else if (request.method === "GET") {
    console.log("Fetching data");
    const data = await getTrackerData();
    console.log("Fetched data");
    const headers = {
      ...corsHeaders,
      "Content-Type": "application/json",
    };
    return new Response(JSON.stringify(convertToNumber(data)), {
      status: 200,
      headers: headers,
    });
  }
  return new Response("Method not allowed", { status: 405, headers: corsHeaders });
};

export const endpoints: Record<string, EndpointHandler> = {
  "/data": handler,
};
