<script type="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { newAddress, type Address } from "./address";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let selectedItem: Address;
    export let entries: Address[];

    function deleteAddress(address: Address) {
        selectedItem = undefined;
        invoke("delete_address", { address: address });
        dispatch("refresh");
        console.log("delete");
        event.stopPropagation()
    }

    function addNewAddress() {
        selectedItem = newAddress();
    }

    function selectItem(item: Address) {
        selectedItem = item;
        console.log("select");
    }
</script>

<div class="panel">
    <p class="panel-heading">Addresses</p>

    <div class="panel-block">
        <p class="control has-icons-left">
            <input class="input" type="text" placeholder="Search" />
        </p>
    </div>

    {#each entries as item}
        <a class="panel-block has-icons-right is-justify-content-space-between" on:click={() => selectItem(item)}>
            {item.name}
            <span class="icon is-right">
                <button class="delete" on:click={() => deleteAddress(item)} />
            </span>
        </a>
    {/each}

    <div class="panel-block">
        <button class="button is-info" on:click={addNewAddress}>Add</button>
    </div>
</div>
