<script type="ts">
    export let selectedItem;
    export let entries;
</script>


<div class="panel">
    <p class="panel-heading">Addresses</p>

    <div class="panel-block">
        <p class="control has-icons-left">
            <input class="input" type="text" placeholder="Search" />
        </p>
    </div>

    {#each entries as item}
        <a class="panel-block" on:click="{() => selectedItem = item}">
            {item.name}
        </a>
    {/each}

    <div class="panel-block">
        <button class="button is-info">Add</button>
    </div>
</div>
