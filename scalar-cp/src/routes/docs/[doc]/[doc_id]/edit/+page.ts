import type { Schema } from "$ts/Schema";
import type { Item } from "$ts/Item";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import { apiFetch } from "$lib/api";
import { base } from "$app/paths";

export const load: PageLoad = async ({ params, fetch }) => {
  let schema: Schema = await (
    await apiFetch(fetch, `${base}/api/docs/${params.doc}/schema`)
  ).json();
  let doc_request = await apiFetch(
    fetch,
    `${base}/api/docs/${params.doc}/${params.doc_id}`,
  );

  let doc: Item = await doc_request.json();

  return {
    schema,
    doc: doc.content,
  };
};
