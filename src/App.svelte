<script lang="ts">
    import '../theme/theme.scss';

    import { Client } from "./client";
    import SplitPane from './SplitPane.svelte';
    import RequestList from './RequestList.svelte';

    import { activeRequest, requests, setActiveRequest } from "./state";
    import RequestResponseContainer from "./RequestResponseContainer.svelte";
    import { newRequestState } from "./types";

    let client = new Client();

    function addNewRequest() {
        const req = newRequestState('GET', 'https://blq.me');
        requests.update(curr => [...curr, req]);
        setActiveRequest($activeRequest, req);
    }

    $: if ($requests.length > 0 && !$activeRequest) {
        activeRequest.update(_ => $requests[0]);
    }

    // onMount(async () => {
    //     console.log(await select("select ?, ?;", ...[1, 2]));
    //     console.log(await selectOne("select ?, ?;", 1, 2));
    //     console.log(await exec("select ?, ?;", 1, 2));
    // })
</script>

<main>
    <div class="header navbar is-primary is-radiusless">
        <button class="button is-primary is-inverted" on:click={addNewRequest}>
            <span class="icon is-medium">
                <i class="far fa-plus"></i>
            </span>
            <span>New</span>
        </button>
    </div>

    <div class="list-editor-container">
        {#if $activeRequest}
            <SplitPane type="horizontal" pos="25" class="split-pane">
                <section slot=a class="split-section request-list-section">
                    <RequestList />
                </section>

                <section slot=b class="split-section request-editor-section">
                    <RequestResponseContainer />
                </section>
            </SplitPane>
        {:else}
            <div class="no-request-container">
                <h4>No request selected.</h4>
            </div>
        {/if}
    </div>
</main>

<style>
	main {
		margin: 0;
        height: 100vh;
        display: flex;
        flex-direction: column;
	}
    .header {
        height: 55px;
        min-height: 55px;
        display: flex;
        flex-direction: row;
        align-items: center;
        padding-left: 10px;
        padding-right: 10px;
    }
    .split-section {
        height: 100%;
    }
    .list-editor-container {
        flex: auto;
        display: flex;
        flex-direction: column;
    }
    .request-list-section {
        overflow-y: auto;
    }
    .no-request-container {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .no-request-container > h4 {
        text-align: center;
    }
	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
