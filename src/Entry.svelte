<script type="ts">
    import type { Address } from "./address";
    import { invoke } from "@tauri-apps/api/tauri";
    import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

    export let shownItem: Address;

    function saveAddress(){
        invoke("save_address", { address: shownItem });
        dispatch("refresh");
    }
</script>

{#if shownItem}
    <div class="field">
        <div class="label">Name</div>
        <div class="control">
            <input class="input" type="text" placeholder="Text input" bind:value={shownItem.name} />
        </div>
    </div>

    <div class="field">
        <div class="label">Email</div>
        <div class="control">
            <input class="input is-danger" type="email" placeholder="Email input" bind:value={shownItem.name} />
        </div>
    </div>

    <div class="field">
        <div class="control">
            <label class="radio">
                <input type="radio" name="question" />
                Male
            </label>
            <label class="radio">
                <input type="radio" name="question" />
                Female
            </label>
        </div>
    </div>

    <div class="field">
        <div class="label">Address</div>
        <div class="control">
            <textarea class="textarea" placeholder="Address" bind:value={shownItem.address} />
        </div>
    </div>

    <div class="field">
        <div class="control">
            <label class="checkbox">
                <input type="checkbox" />
                I don't give a fuck about the terms and conditions
            </label>
        </div>
    </div>

    <div class="field is-grouped">
        <div class="control">
            <button class="button is-link" on:click={saveAddress}>Save</button>
        </div>
    </div>
{/if}
