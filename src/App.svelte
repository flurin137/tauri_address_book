<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import type { Address } from "./address";
    import Entries from "./Entries.svelte";
    import Entry from "./Entry.svelte";

    let selectedItem: Address;
    let addresses: Address[];

    $: console.log(selectedItem);

    onMount(async () => {
        addresses = await invoke("get_addresses");
    });

    async function update() {
        addresses = await invoke("get_addresses");
    }
</script>

<main class="columns">
    <div class="column is-narrow">
        {#if addresses}
            <Entries bind:selectedItem entries={addresses} on:refresh={update}/>
        {:else}
            <p>...waiting</p>
        {/if}
    </div>
    <div class="column">
        <Entry bind:shownItem={selectedItem} on:refresh={update} />
    </div>
</main>

<style>
    :root {
        height: 100%;
    }

    main {
        text-align: center;
        padding: 1em;
        margin: 0 auto;
        min-height: 100%;
        vertical-align: inherit;
        overflow-y: auto;
    }
</style>
