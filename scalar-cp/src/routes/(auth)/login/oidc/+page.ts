import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ url, fetch }) => {
  console.log(url);
  let response = await fetch(
    `/api/signin/oidc/complete?code=${url.searchParams.get("code")}&state=${url.searchParams.get("state")}`,
  );
  console.log(response);

  if (response.status == 200) {
    return {
      token: await response.text(),
    };
  }

  return {
    error: response.status,
  };
};
