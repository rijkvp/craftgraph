<script lang="ts">
	import type { PageData } from './$types';
    import init, { add } from '../../../../pkg/craftgraph';
	
	export let data: PageData;

    let result = 0;
    init().then(() => {
        result = add(3, 5);
    });
</script>

<h1>{data.item}</h1>

<h2>3 + 5 = {result}</h2>
