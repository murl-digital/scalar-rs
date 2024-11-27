import { apiFetch } from "$lib/server/api";
import type { RequestHandler } from "@sveltejs/kit";

export const PATCH: RequestHandler = async ({ params, request }) => {
  let text = await request.json();
  console.log("bruh");
  const response = await apiFetch(
    fetch,
    `http://localhost:3000/cpanel/docs/${params.doc}/drafts/${params.doc_id}`,
    {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(text),
    },
  );

  return response;
};
