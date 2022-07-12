<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import type { Address } from "./address";
    import Entries from "./Entries.svelte";
    import Entry from "./Entry.svelte";

    import { listen } from "@tauri-apps/api/event";

    let selectedItem: Address;
    let addresses: Address[];

    $: console.log(selectedItem);

    onMount(async () => {
        addresses = await invoke("get_addresses");
        await invoke("init_process");
        await listen("Yeeehaaa", (event) => {
            console.log(event);
        });
    });

    async function update() {
        addresses = await invoke("get_addresses");
    }
</script>

<div class="drawer drawer-mobile">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />

    <div class="drawer-content flex flex-col items-center justify-start">
        {#if selectedItem}
            <Entry bind:shownItem={selectedItem} on:refresh={update} />
        {:else}
            <p>ASDJKHASKJH ASDJKH</p>
        {/if}
    </div>

    {#if addresses}
        <Entries bind:selectedItem entries={addresses} on:refresh={update} />
    {:else}
        <p>...waiting</p>
    {/if}
</div>

<main class="container" />

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
