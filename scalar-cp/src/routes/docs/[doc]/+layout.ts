import { error } from '@sveltejs/kit';
import type {PageLoad} from './$types';

export const load: PageLoad = async ({params, fetch}) => {
    let req = await fetch(`http://localhost:3000/docs/${params.doc}`);

    if (req.status == 404) {
        throw error(404)
    }

    let docs = await req.json()

    return {
        docs: docs
    }
}