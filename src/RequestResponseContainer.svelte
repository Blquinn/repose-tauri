<script lang="ts">
    import Dropdown from "./Dropdown.svelte";
    import RequestEditor from "./RequestEditor.svelte";
    import ResponseViewer from "./ResponseViewer.svelte";
    import {methods, RequestResponseDirection} from "./types";
    import {activeRequest, updateRequest} from "./state";
    import {defaultClient} from "./client";
    import type {HttpResponse} from "./tauri/http";

    async function sendRequest() {
        if (!$activeRequest) return;

        activeRequest.update(req => { return {
            ...req,
            isLoading: true,
            requestResponseDirection: RequestResponseDirection.Response,
        }});

        const params = $activeRequest?.params.filter(p => p.key !== '').map(p => [p.key, p.value]);
        const headers = $activeRequest?.headers.filter(p => p.key !== '').map(p => [p.key, p.value]);

        const res: HttpResponse = await defaultClient.request($activeRequest.method, $activeRequest.url, {
            params,
            headers
        });

        activeRequest.update(r => { return {
            ...r,
            isLoading: false,
            lastResponse: res,
        }});
    }
</script>

<main>
    <div class="request-details-container">
        <input type="text" class="input name-input" placeholder="Name"
               bind:value={$activeRequest.name}
               on:change={() => updateRequest($activeRequest)}
        >
        <div class="request-details-bar">
            <Dropdown options={methods} bind:activeValue={$activeRequest.method} />
            <input type="text" class="input" placeholder="Url" bind:value={$activeRequest.url}>
            <button class="button is-link" on:click={sendRequest}>Send</button>
            <button class="button is-link">Save</button>
        </div>
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
    .request-details-container {
        padding: 5px;
    }
    .request-details-bar {
        display: flex;
        flex-direction: row;
        margin-top: 5px;
    }
    .request-response-buttons {
        margin-top: 5px;
        margin-bottom: 0;
    }
    .request-response-buttons button {
        min-width: 100px;
    }
</style>
