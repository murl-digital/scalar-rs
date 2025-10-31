import { goto } from "$app/navigation";
import { base } from "$app/paths";
import { redirect } from "@sveltejs/kit";
import { state } from "$lib/state.svelte";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
  if (state.authenticated) {
    await goto(`${base}/`);
  }

  let oidc = await fetch(`${base}/api/signin/oidc/is_auto`);

  if (oidc.status == 200) {
    let is_auto: boolean = await oidc.json();
    if (is_auto) {
      redirect(303, `${base}/api/signin/oidc`);
    }
  }
};
