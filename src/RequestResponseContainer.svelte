<script lang="ts">
    import Dropdown from "./Dropdown.svelte";
    import RequestEditor from "./RequestEditor.svelte";
    import ResponseViewer from "./ResponseViewer.svelte";
    import { methods } from "./types";
    import { activeRequest } from "./state";

    enum ActiveDirection { Request, Response }

    let direction = ActiveDirection.Request;
</script>

<main>
    <div class="request-details-bar">
        <Dropdown options={methods} activeValue={$activeRequest.method} />
        <input type="text" class="input" placeholder="Url" value={$activeRequest.url}>
        <button class="button is-link">Send</button>
        <button class="button is-link">Save</button>
    </div>

    <div class="request-response-buttons buttons is-centered has-addons">
        <button class="button {
                direction === ActiveDirection.Request ? 'is-info is-selected' : ''}"
                on:click={() => direction = ActiveDirection.Request}
        >Request</button>
        <button class="button {direction === ActiveDirection.Response ? 'is-info is-selected' : ''}"
                on:click={() => direction = ActiveDirection.Response}
        >Response</button>
    </div>

    <!-- Use show to prevent instantiating the components repeatedly. -->
    <RequestEditor show={direction === ActiveDirection.Request} />
    <ResponseViewer show={direction === ActiveDirection.Response} />
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
</style>
