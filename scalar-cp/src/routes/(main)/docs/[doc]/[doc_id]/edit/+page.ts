import type { Schema } from "$ts/Schema";
import type { Item } from "$ts/Item";
import type { PageLoad } from "./$types";
import { apiFetch } from "$lib/api";
import { base } from "$app/paths";

export const load: PageLoad = async ({ params, fetch }) => {
  let doc_request = await apiFetch(
    fetch,
    `${base}/api/docs/${params.doc}/${params.doc_id}`,
  );

  let doc: Item = await doc_request.json();

  return {
    doc: doc.content,
  };
};
