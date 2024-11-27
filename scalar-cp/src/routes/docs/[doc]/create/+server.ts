import { apiFetch } from "$lib/server/api";
import { redirect, type RequestHandler } from "@sveltejs/kit";
import { nanoid } from "nanoid";

export const POST: RequestHandler = async ({ params, request }) => {
  let text = await request.text();
  console.log("bruh");
  let id = nanoid();
  await apiFetch(
    fetch,
    `http://localhost:3000/cpanel/docs/${params.doc}/drafts/${id}`,
    {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: text,
    },
  );

  return new Response(id, { status: 200 });
};
