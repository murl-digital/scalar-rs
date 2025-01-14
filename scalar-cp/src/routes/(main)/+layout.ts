import type { LayoutLoad } from "./$types";
import type { DocInfo } from "$ts/DocInfo";
import { apiFetch } from "$lib/api";
import { state } from "$lib/state.svelte";
import { base } from "$app/paths";

export const ssr = false;

const size = 80;

export const load: LayoutLoad = async ({ params, fetch }) => {
  let docs: DocInfo[] = [];
  let avatarUrl;
  if (state.authenticated) {
    let user = await apiFetch(fetch, `${base}/api/me`).then((r) => r.json());
    avatarUrl = `https://www.gravatar.com/avatar/${user.gravatar_hash}?s=${size}&d=identicon`;
    docs = await (await apiFetch(fetch, `${base}/api/docs`)).json();
  }

  return {
    docs: docs,
    avatarUrl,
  };
};
