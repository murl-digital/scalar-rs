import type {PageLoad} from './$types';

export const load: PageLoad = async ({params, fetch}) => {
    let docs = await (await fetch("http://localhost:3000/docs")).json()

    return {
        docs: docs
    }
}