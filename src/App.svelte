<script lang="ts">
    import '../theme/theme.scss';

    import {Client} from "./client";
    import SplitPane from './SplitPane.svelte';
    import RequestList from './RequestList.svelte';
    import RequestEditor from './RequestEditor.svelte';
    import ResponseViewer from "./ResponseViewer.svelte";

    import {writable} from "svelte/store";
    import type {RootState} from "./state";

    let rootState = writable<RootState>({
        clicked: '',
    });

    let client = new Client();

    let editor: any = null;

    enum ActiveDirection {
        Request,
        Response
    }

    let direction = ActiveDirection.Request;

    enum ActiveTab {
        Params = 'Params',
        Headers = 'Headers',
        Body = 'Body',
    }

    const methods = [
        'GET',
        'POST',
        'PUT',
        'PATCH',
        'DELETE',
        'HEAD',
        'OPTIONS',
    ];
</script>

<main>
    <div class="header navbar is-primary is-radiusless">
        <button class="button is-primary is-inverted">
            <span class="icon is-medium">
                <i class="far fa-plus"></i>
            </span>
            <span>New</span>
        </button>
    </div>

    <div class="list-editor-container">
        <SplitPane type="horizontal" pos="25" class="split-pane">
            <section slot=a class="split-section request-list-section">
                <RequestList state="{rootState}" />
            </section>

            <section slot=b class="split-section request-editor-section">
                <div class="request-details-bar">
                    <div class="select" id="method-select">
                        <select>
                            {#each methods as method}
                                <option>{method}</option>
                            {/each}
                        </select>
                    </div>
                    <input type="text" class="input" placeholder="Url">
                    <button class="button is-link">Send</button>
                    <button class="button is-link">Save</button>
                </div>

                <div class="request-response-buttons buttons is-centered has-addons">
                    <button
                            class="button {direction === ActiveDirection.Request ? 'is-info is-selected' : ''}"
                            on:click={() => direction = ActiveDirection.Request}
                    >Request</button>
                    <button
                            class="button {direction === ActiveDirection.Response ? 'is-info is-selected' : ''}"
                            on:click={() => direction = ActiveDirection.Response}
                    >Response</button>
                </div>

                <!-- Use show to prevent instantiating the components repeatedly. -->
                <RequestEditor show={direction === ActiveDirection.Request} />
                <ResponseViewer show={direction === ActiveDirection.Response} />
            </section>
        </SplitPane>
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
        overflow-y: hidden;
        display: flex;
        flex-direction: column;
    }
    .request-list-section {
        overflow-y: auto;
    }
    .split-pane {
        max-height: 100%;
        flex-grow: 1;
    }
    .request-details-bar {
        padding: 5px;
        display: flex;
        flex-direction: row;
    }
    .request-details-bar > * {
        margin-left: 2px;
        margin-right: 2px;
    }
    .request-response-buttons {
        margin-top: 5px;
        margin-bottom: 0;
    }
    .request-editor-section {
        display: flex;
        flex-direction: column;
    }
    .request-editor-section > * {
        flex-shrink: 0;
    }
	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
