import { error } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";
import { apiFetch } from "$lib/api";
import { base } from "$app/paths";

export const load: LayoutLoad = async ({ params, fetch }) => {
  let req = await apiFetch(fetch, `${base}/api/docs/${params.doc}`);

  if (req.status == 404) {
    throw error(404);
  }

  let docs = await req.json();

  console.log(docs);

  return {
    docs: docs,
  };
};
