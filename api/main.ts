import { endpoints } from "./src/endpoints.ts";
// origins allowed to get data 
// TODO: set cors
const allowedOrigins = ["https://johncarlomanuel.com/"]


const handler = async (request: Request): Promise<Response> => {
  const url = new URL(request.url);
  const endpoint = endpoints[url.pathname];
  if (endpoint) {
    return await endpoint(request);
  }
  return new Response("Not found", { status: 404 });
}

if (import.meta.main) {
  console.log("Starting server...");
  Deno.serve({
    port: 8000,
    handler: (request) => handler(request),
  });
}
