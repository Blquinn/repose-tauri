<script lang="ts">
    import Dropdown from "./Dropdown.svelte";
    import RequestEditor from "./RequestEditor.svelte";
    import ResponseViewer from "./ResponseViewer.svelte";
    import {methods, RequestResponseDirection} from "./types";
    import {activeRequest} from "./state";
    import {defaultClient} from "./client";
    import type {HttpResponse} from "./tauri/http";

    async function sendRequest() {
        const res: HttpResponse = await defaultClient.request($activeRequest.method, $activeRequest.url);
        activeRequest.update(r => { return {
            ...r,
            isLoading: false,
            requestResponseDirection: RequestResponseDirection.Response,
            lastResponse: res,
        }});
    }
</script>

<main>
    <div class="request-details-bar">
        <Dropdown options={methods} bind:activeValue={$activeRequest.method} />
        <input type="text" class="input" placeholder="Url" bind:value={$activeRequest.url}>
        <button class="button is-link" on:click={sendRequest}>Send</button>
        <button class="button is-link">Save</button>
    </div>

    <div class="request-response-buttons buttons is-centered has-addons">
        <button class="button {$activeRequest.requestResponseDirection === RequestResponseDirection.Request ? 'is-info is-selected' : ''}"
                on:click={() => activeRequest.update(curr => { return {...curr, requestResponseDirection: RequestResponseDirection.Request}})}
        >Request</button>
        <button class="button {$activeRequest.requestResponseDirection === RequestResponseDirection.Response ? 'is-info is-selected' : ''}"
                on:click={() => activeRequest.update(curr => { return {...curr, requestResponseDirection: RequestResponseDirection.Response}})}
        >Response</button>
    </div>

    <!-- Use show to prevent instantiating the components repeatedly. -->
    <RequestEditor show={$activeRequest.requestResponseDirection === RequestResponseDirection.Request} />
    <ResponseViewer show={$activeRequest.requestResponseDirection === RequestResponseDirection.Response} />
</main>

<style>
    main {
        height: 100%;
        display: flex;
        flex-direction: column;
    }
    main > * {
        flex-shrink: 0;
    }
    .request-details-bar > * {
        margin-left: 2px;
        margin-right: 2px;
    }
    .request-details-bar {
        padding: 5px;
        display: flex;
        flex-direction: row;
    }
    .request-response-buttons {
        margin-top: 5px;
        margin-bottom: 0;
    }
    .request-response-buttons button {
        min-width: 100px;
    }
</style>
