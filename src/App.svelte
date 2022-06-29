<script lang="ts">
    import Entries from "./Entries.svelte";
    import Entry from "./Entry.svelte";

    let selectedItem;
</script>

<main class="columns">
    <div class="column is-narrow">
        <Entries bind:selectedItem />
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
