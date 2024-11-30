import type { Schema } from "$ts/Schema";
import { apiFetch } from "$lib/api";
import type { PageLoad } from "./$types";
import { base } from "$app/paths";

export const load: PageLoad = async ({ params, fetch }) => {
  let schema: Schema = await (
    await apiFetch(fetch, `${base}/api/docs/${params.doc}/schema`)
  ).json();

  return {
    schema: schema,
  };
};
