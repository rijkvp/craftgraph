import type { LayoutLoad } from './$types';
import init, { loadItems } from '../../../pkg/craftgraph';
	
export const ssr = false;

export const load: LayoutLoad = async () => {
    await init();
    const items = await loadItems();
    return { items: items };
};
