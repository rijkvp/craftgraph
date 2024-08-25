import type { LayoutLoad } from './$types';

export const ssr = false;

export const load: LayoutLoad = async ({ fetch }) => {
    const res = await fetch("/data.json");
    const data = await res.json();
    return data;
};
