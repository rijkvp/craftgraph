<script lang="ts">
    export let items: string[] = [];
    export let onLookup: (input: string) => void;

    function lookupItem(e: InputEvent) {
        const input = (e.target as HTMLInputElement).value;
        if (items.includes(input)) {
            onLookup(input);
        } 
    }

</script>

<div>
    <input type="text" placeholder="Search for item.." on:change={lookupItem} />
</div>
