import type { Schema } from "$ts/Schema";
import type { Item } from "$ts/Item";
import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";
import { apiFetch } from "$lib/server/api";

export const load: PageServerLoad = async ({ params, fetch }) => {
  let schema: Schema = await (
    await apiFetch(
      fetch,
      `http://localhost:3000/cpanel/docs/${params.doc}/schema`,
    )
  ).json();
  let doc_request = await apiFetch(
    fetch,
    `http://localhost:3000/cpanel/docs/${params.doc}/${params.doc_id}`,
  );

  let doc: Item = await doc_request.json();

  return {
    schema,
    doc: doc.content,
  };
};
