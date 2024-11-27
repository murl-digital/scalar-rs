import type { Schema } from "$ts/Schema";
import { apiFetch } from "$lib/server/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ params, fetch }) => {
  let schema: Schema = await (
    await apiFetch(
      fetch,
      `http://localhost:3000/cpanel/docs/${params.doc}/schema`,
    )
  ).json();

  return {
    schema: schema,
  };
};
