<script lang="ts">
	import '../app.css';
    import type { LayoutData } from './$types';
    import { goto } from '$app/navigation';

    import ItemSelector from "$lib/ItemSelector.svelte";
	
	export let data: LayoutData;
</script>

<main>
    <h1>CraftGraph</h1>
    <p>{data.items.length} items</p>
    <ItemSelector items={data.items} onLookup={(input) => goto(`/${input}`)} />
    <slot/>
</main>
