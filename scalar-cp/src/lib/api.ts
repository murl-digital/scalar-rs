import { browser } from "$app/environment";
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import { error, redirect } from "@sveltejs/kit";
import { get } from "svelte/store";
import { state } from "./state.svelte";

export async function apiFetch(
  fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
  input: RequestInfo | URL,
  init?: RequestInit,
): Promise<Response> {
  if (!browser) {
    error(500, "tried to run an api fetch in the server");
  }

  let token = sessionStorage.getItem("token");

  let patched_init: RequestInit = init ?? { headers: {} };
  if (Array.isArray(patched_init.headers)) {
    patched_init.headers.push(["Authorization", "Bearer "]);
  } else if (patched_init.headers instanceof Headers) {
    patched_init.headers.append("Authorization", `Bearer ${token}`);
  } else if (patched_init.headers) {
    patched_init.headers["Authorization"] = `Bearer ${token}`;
  }

  let response = await fetch(input, patched_init);
  console.log(get(page));
  if (response.status === 401) {
    state.authenticated = false;
    if (get(page).route?.id !== "/login") {
      goto("/login");
    }
  }

  return response;
}
