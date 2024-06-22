import type { Schema } from '$lib/Schema';
import type {PageLoad} from './$types';

export const load: PageLoad = async ({params, fetch}) => {
    let schema: Schema = await (await fetch(`http://localhost:3000/docs/${params.slug}/schema`)).json()

    return {
        schema: schema
    }
}