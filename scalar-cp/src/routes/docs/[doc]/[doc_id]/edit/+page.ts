import type { Schema } from '$lib/Schema';
import { error } from '@sveltejs/kit';
import type {PageLoad} from './$types';

export const load: PageLoad = async ({params, fetch}) => {
    let schema: Schema = await (await fetch(`http://localhost:3000/docs/${params.doc}/schema`)).json()
    let doc_request = await fetch(`http://localhost:3000/docs/${params.doc}/${params.doc_id}`);

    if (doc_request.status == 404) {
        throw error(404)
    }

    let doc = await doc_request.json();

    return {
        schema,
        doc
    }
}