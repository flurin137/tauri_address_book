<script lang="ts">
    import Entries from "./Entries.svelte";
    import Entry from "./Entry.svelte";
</script>

<main class="container">
    <div class="columns">
        <div class="column is-3">
            <Entries />
        </div>
        <div class="column">
            <Entry />
        </div>
    </div>
</main>
