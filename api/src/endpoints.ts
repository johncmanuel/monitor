import { Data } from "../types/data.d.ts";
import { storeTrackerData } from "../src/kv.ts";

type EndpointHandler = (request: Request) => Promise<Response>;

export const trackerHandler: EndpointHandler = async (request) => {
  if (
    request.headers.get("Authorization") !==
      `Bearer ${Deno.env.get("API_KEY")!}`
  ) {
    return new Response("Unauthorized", { status: 401 });
  }
  if (request.method === "POST") {
    const data: Data = await request.json();
    console.log(data);
    await storeTrackerData(data);
    return new Response("Data received", { status: 200 });
  }
  return new Response("Method not allowed", { status: 405 });
};

export const endpoints: Record<string, EndpointHandler> = {
  "/tracker": trackerHandler,
};
