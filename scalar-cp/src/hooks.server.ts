import { redirect, type Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";
import jwt, { type JwtPayload } from "jsonwebtoken";

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

const auth: Handle = async ({ event, resolve }) => {
  if (event.url.pathname !== "/login") {
    let token: string = event.cookies.get("token") ?? redirect(302, "/login");
    let nonce: string = event.cookies.get("nonce") ?? redirect(302, "/login");

    try {
      const payload: JwtPayload = jwt.verify(
        token,
        "REALLYBADCHANGELATER",
      ) as JwtPayload;
      if (
        new Bun.CryptoHasher("sha256").update(nonce).digest("base64") !==
        payload.nonce
      ) {
        console.log(
          new Bun.CryptoHasher("sha256").update(nonce).digest("base64"),
        );
        console.log(payload.nonce);
        throw "sex";
      }

      event.locals.email = payload.username;
    } catch (e) {
      event.cookies.delete("token", { path: "/" });
      event.cookies.delete("nonce", { path: "/" });
      redirect(302, "/login");
    }
  }

  const response = await resolve(event);
  return response;
};

export const handle: Handle = sequence(auth, unocss);
