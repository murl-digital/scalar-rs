import { type Handle } from "@sveltejs/kit";

const unocss: Handle = async ({ event, resolve }) => {
  const response = await resolve(event, {
    transformPageChunk: ({ html }) =>
      html.replace(
        "%unocss-svelte-scoped.global%",
        "unocss_svelte_scoped_global_styles",
      ),
  });
  return response;
};

export const handle: Handle = unocss;
