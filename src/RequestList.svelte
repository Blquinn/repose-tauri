<script lang="ts">
    import {requests, activeRequest, setActiveRequest} from "./state";

    let filter: string = '';
</script>

<main>
    <div class="input-wrap">
        <div class="field">
            <p class="control has-icons has-icons-right">
                <input class="input is-rounded search-input"
                       type="email" placeholder="Filter" bind:value={filter}>
                <span class="icon is-small is-right">
                    <i class="fas fa-search"></i>
                </span>
            </p>
        </div>
    </div>

    <aside class="menu">
        <ul class="menu-list">
            {#each $requests as request (request.id)}
                {#if filter === '' || request.name.toLowerCase().includes(filter.toLowerCase())}
                    <li on:click={() => { setActiveRequest($activeRequest, request) }}>
                        <a class="{request.id === $activeRequest.id ? 'is-active' : ''}" href="/#"
                        >{request.name === '' ? 'New Request' : request.name}</a>
                    </li>
                {/if}
            {/each}
        </ul>
    </aside>
</main>

<style>
    .input-wrap {
        padding: 5px 5px 5px 5px;
    }
    .menu-list li a {
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
    }
</style>
