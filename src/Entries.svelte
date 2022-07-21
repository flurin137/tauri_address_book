<script type="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { newAddress, type Address } from "./address";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let selectedItem: Address;
    export let entries: Address[];
    export let filteredEntries: Address[];

    let searchString: string = "";

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

    function searchPress(e: { key: string }) {
        if (e.key === "Escape") {
            searchString = "";
        }
    }

    $: filteredEntries =
        searchString == ""
            ? entries
            : entries.filter((d) => d.name.indexOf(searchString) >= 0);
</script>

<div class="flex flex-col grow">
    <div class="input-group flex">
        <input
            type="text"
            bind:value={searchString}
            placeholder="Search…"
            class="input input-bordered grow"
            on:keydown={searchPress}
        />
        <button class="btn btn-square">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                />
            </svg>
        </button>
    </div>

    <ul class="menu grow overflow-y-scroll overscroll-contain my-2 scrollbar">
        {#each filteredEntries as item}
            <li on:click={() => selectItem(item)}>
                <div class="flex">
                    <div class="flex-auto">
                        {item.name}
                    </div>
                    <div class="w-3 flex-none mr-2">
                        <button
                            class="btn btn-square btn-xs"
                            on:click={() => deleteAddress(item)}
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-3 w-3"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                                ><path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12"
                                /></svg
                            >
                        </button>
                    </div>
                </div>
            </li>
        {/each}
    </ul>

    <button class="btn is-info" on:click={addNewAddress}>Add</button>
</div>
