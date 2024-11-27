import type { Actions } from "./$types";
import jwt from "jsonwebtoken";
import { randomBytes, randomFillSync } from "node:crypto";

export const actions = {
  default: async ({ cookies, request, locals }) => {
    const formData = await request.formData();
    const email = formData.get("email");
    const password = formData.get("password");

    if (email === "contact@draconium.productions" && password === "gaysex") {
      let nonce = randomBytes(256).toString("base64");
      cookies.set(
        "token",
        jwt.sign(
          {
            username: "contact@draconium.productions",
            nonce: new Bun.CryptoHasher("sha256")
              .update(nonce)
              .digest("base64"),
          },
          "REALLYBADCHANGELATER",
        ),
        {
          path: "/",
        },
      );
      cookies.set("nonce", nonce, { path: "/" });
      locals.email = email;
    }
  },
} satisfies Actions;
