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
        event.stopPropagation();
    }

    function addNewAddress() {
        selectedItem = newAddress();
    }

    function selectItem(item: Address) {
        selectedItem = item;
        console.log("select");
    }
</script>

<div class="drawer-side">
    <ul class="menu p-4 overflow-y-auto w-80 bg-base-100 text-base-content">
        <div class="form-control">
            <div class="input-group">
                <input type="text" placeholder="Search…" class="input input-bordered" />
                <button class="btn btn-square">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                    </svg>
                </button>
            </div>
        </div>

        {#each entries as item}
            <li>
                <a on:click={() => selectItem(item)}>
                    {item.name}
                    <span class="icon is-right">
                        <button class="delete" on:click={() => deleteAddress(item)} />
                    </span>
                </a>
            </li>
        {/each}
        
        <div class="panel-block">
            <button class="btn is-info" on:click={addNewAddress}>Add</button>
        </div>
    </ul>
</div>
