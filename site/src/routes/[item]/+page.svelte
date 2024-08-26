<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<h1>{data.item}</h1>

<p>Graph: {JSON.stringify(data.craftGraph, null, 4)}</p>

<style>
    p {
        white-space: pre-wrap;
        text-align: left;
        font-family: monospace;
    }
</style>
