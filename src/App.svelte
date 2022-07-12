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

<div class="flex grow gap-4">
    <side class="flex w-80 flex-none">
        {#if addresses}
            <Entries bind:selectedItem entries={addresses} on:refresh={update} />
        {/if}
    </side>

    <div class="grow">
        {#if selectedItem}
            <div class="flex flex-col items-start">
                <Entry bind:shownItem={selectedItem} on:refresh={update} />
            </div>
        {:else}
            <div class="flex justify-center items-center">
                <p>ASDJKHASKJH ASDJKH</p>
            </div>
        {/if}
    </div>
</div>

<style>
    :root {
        height: 100%;
    }

</style>
