import type { LayoutServerLoad } from "./$types";
import type { DocInfo } from "$ts/DocInfo";
import { apiFetch } from "$lib/server/api";

//export const ssr = false;

const size = 80;

export const load: LayoutServerLoad = async ({ params, fetch, locals }) => {
  let docs: DocInfo[] = [];

  let avatarUrl;

  console.log(locals);
  console.log(docs);

  if (locals.email) {
    const hash = new Bun.CryptoHasher("sha256")
      .update(locals.email)
      .digest("hex");
    avatarUrl = `https://www.gravatar.com/avatar/${hash}?s=${size}&d=identicon`;
    docs = await (
      await apiFetch(fetch, "http://localhost:3000/cpanel/docs")
    ).json();
  }

  return {
    docs: docs,
    avatarUrl,
  };
};
