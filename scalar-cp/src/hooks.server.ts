import { type Handle } from "@sveltejs/kit";

const unocss: Handle = async ({ event, resolve }) => {
  const response = await resolve(event, {
    transformPageChunk: ({ html }) =>
      html
        .replace("%splash.head%", `_vite_splash_head_`)
        .replace("%splash.body%", `_vite_splash_body_`),
  });
  return response;
};

export const handle: Handle = unocss;
