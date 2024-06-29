import type {PageLoad} from './$types';
import type {DocInfo} from '$ts/DocInfo';

export const load: PageLoad = async ({params, fetch}) => {
    let docs: DocInfo = await (await fetch("http://localhost:3000/docs")).json()

    return {
        docs: docs
    }
}