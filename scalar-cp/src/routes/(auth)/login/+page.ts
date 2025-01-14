import { goto } from "$app/navigation";
import { base } from "$app/paths";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ parent }) => {
  let data = await parent();

  if (data.avatarUrl) {
    await goto(`${base}/`);
  }
};
