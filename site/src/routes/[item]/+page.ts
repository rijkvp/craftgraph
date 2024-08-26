import type { PageLoad } from './$types';
import { getCraftGraph } from '../../../../pkg/craftgraph';

export const load: PageLoad = async ({ params }) => {
    const craftGraph = await getCraftGraph(params.item);
    console.log(craftGraph);
    return { item: params.item, craftGraph: craftGraph };
};
