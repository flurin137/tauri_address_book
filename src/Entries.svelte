<script type="ts">
    export let items = ["asdas", "12312", "asdas", "qwe", "qweqw", "fdgdf", "dfg"];
</script>

<div class="panel">
    <p class="panel-heading">Addresses</p>

    <div class="panel-block">
        <p class="control has-icons-left">
            <input class="input" type="text" placeholder="Search" />
        </p>
    </div>

    {#each items as item}
        <a class="panel-block">
            {item}
        </a>
    {/each}

    <div class="panel-block">
        <button class="button is-info">Add</button>
    </div>
</div>
