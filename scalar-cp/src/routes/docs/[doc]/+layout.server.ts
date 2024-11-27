import { error } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { apiFetch } from "$lib/server/api";

export const load: LayoutServerLoad = async ({ params, fetch }) => {
  let req = await apiFetch(
    fetch,
    `http://localhost:3000/cpanel/docs/${params.doc}`,
  );

  if (req.status == 404) {
    throw error(404);
  }

  let docs = await req.json();

  console.log(docs);

  return {
    docs: docs,
  };
};
