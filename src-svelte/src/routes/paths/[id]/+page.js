import { error } from '@sveltejs/kit';

export function load({ params }) {
    try {
        let id = Number.parseInt(params.id);
        return {id};
    }
    catch(e) {
        error(404, 'Not found: ' + params);
    }

}