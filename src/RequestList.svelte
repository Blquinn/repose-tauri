<script lang="ts">
    import {requests, activeRequest} from "./state";
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
            {#each $requests as request (request.id)}
                <li on:click={() => { activeRequest.set(request) }}>
                    <a class="{request.id === $activeRequest.id ? 'is-active' : ''}" href="/#"
                    >{request.name ?? 'New Request'}</a>
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
