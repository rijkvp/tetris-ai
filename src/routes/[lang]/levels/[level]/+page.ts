import type { PageLoad } from './$types';
import { levels } from '$lib/levels';

export const load: PageLoad = ({ params }) => {
    return {
        level: levels.find((level) => level.key === params.level)
    };
};
