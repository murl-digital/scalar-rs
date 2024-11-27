import { error, redirect } from "@sveltejs/kit";

export async function apiFetch(
  fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
  input: RequestInfo | URL,
  init?: RequestInit,
): Promise<Response> {
  let patched_init: RequestInit = init ?? { headers: {} };
  if (Array.isArray(patched_init.headers)) {
    patched_init.headers.push(["Authorization", "Bearer "]);
  } else if (patched_init.headers instanceof Headers) {
    patched_init.headers.append(
      "Authorization",
      "Bearer eyJhbGciOiJIUzI1NiJ9.eyJnYXlfc2V4IjoiaGVsbCB5ZWFoIn0.9AY4OA1JKUWb-2rPQveRCIUFyOahooidk47asSU1gzE",
    );
  } else if (patched_init.headers) {
    patched_init.headers["Authorization"] =
      "Bearer eyJhbGciOiJIUzI1NiJ9.eyJnYXlfc2V4IjoiaGVsbCB5ZWFoIn0.9AY4OA1JKUWb-2rPQveRCIUFyOahooidk47asSU1gzE";
  }

  let response = await fetch(input, patched_init);
  if (response.status === 401) {
    error(500, "incorrect API key");
  }

  return response;
}
