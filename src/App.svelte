<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Entries from "./Entries.svelte";
    import Entry from "./Entry.svelte";

    let selectedItem;
    let getAddressesPromise = invoke("get_addresses");
</script>

<main class="columns">
    <div class="column is-narrow">
        {#await getAddressesPromise}
            <p>...waiting</p>
        {:then addresses}
            <Entries bind:selectedItem entries={addresses} />
        {:catch error}
            <p style="color: red">{error.message}</p>
        {/await}
    </div>
    <div class="column">
        <Entry bind:shownItem={selectedItem} />
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
