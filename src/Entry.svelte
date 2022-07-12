<script type="ts">
    import type { Address } from "./address";
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let shownItem: Address;

    function saveAddress() {
        invoke("save_address", { address: shownItem });
        dispatch("refresh");
    }
</script>

{#if shownItem}
    <div class="form-control">
        <div class="form-control">
            <div class="label">Name</div>
            <input class="input input-bordered" type="text" placeholder="Text input" bind:value={shownItem.name} />
        </div>

        <div class="form-control">
            <div class="label">Email</div>
            <input class="input input-bordered" type="email" placeholder="Email input" bind:value={shownItem.name} />
        </div>

        <div class="form-control">
            <label class="label cursor-pointer">
                <span class="label-text">Male</span>
                <input type="radio" name="radio-6" class="radio checked:checkbox-primary" checked />
            </label>
            <label class="label cursor-pointer">
                <span class="label-text">Female</span>
                <input type="radio" name="radio-6" class="radio checked:checkbox-secondary" checked />
            </label>
        </div>

        <div class="form-control">
            <div class="label">Address</div>
            <textarea class="textarea textarea-bordered" placeholder="Address" bind:value={shownItem.address} />
        </div>

        <div class="form-control">
            <label class="label cursor-pointer">
                <span class="label-text">I don't give a fuck about the terms and conditions</span>
                <input type="checkbox" class="checkbox" />
            </label>
        </div>

        <div class="control">
            <button class="btn is-link" on:click={saveAddress}>Save</button>
        </div>
    </div>
{/if}
