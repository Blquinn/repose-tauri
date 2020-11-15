<script lang="ts">
    import type {RootState} from "./state";
    import {writable} from "svelte/store";

    export let state: writable<RootState>;
    let searchText = '';

    interface Request {
        text: string;
    }

    let activeRequest: Request | null = null;

    let requests: Request[] = [
        {text: 'Gray Kittens'},
        {text: 'A Space Rocket'},
        {text: '100 Pounds of Gravel'},
        {text: 'All of the Shrimp'},
        {text: 'A Planet with a Mall'},
    ]
</script>

<main>
    <div class="input-wrap">
        <div class="field">
            <p class="control has-icons has-icons-right">
                <input class="input is-rounded search-input" type="email" placeholder="Filter">
                <span class="icon is-small is-right">
                    <i class="fas fa-search"></i>
                </span>
            </p>
        </div>
    </div>

    <aside class="menu">
        <ul class="menu-list">
            {#each requests as request}
                <li on:click={() => { activeRequest = request; }}>
                    <a class="{request === activeRequest ? 'is-active' : ''}">{request.text}</a>
                </li>
            {/each}
        </ul>
    </aside>
</main>

<style>
    .input-wrap {
        padding: 5px 5px 5px 5px;
    }
</style>
