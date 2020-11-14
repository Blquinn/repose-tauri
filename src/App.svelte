<script lang="ts">
    import {Client} from "./client";
    import SplitPane from './SplitPane.svelte';
    import RequestList from './RequestList.svelte';
    import CodeMirror from './CodeMirror.svelte';
    import Button from '@smui/button';

    import {writable} from "svelte/store";
    import type {RootState} from "./state";
    import {onMount} from "svelte";

    let rootState = writable<RootState>({
        clicked: '',
    });

    let client = new Client();

    let editor: any = null;

    onMount(() => {
        editor.set('', 'json');
    });
</script>

<main>
    <div class="header">
        <button class="button is-primary">New Request</button>
    </div>

    <div class="list-editor-container">
        <SplitPane type="horizontal" pos="25" class="split-pane">
            <section slot=a class="split-section request-list-section">
                <RequestList state="{rootState}" />
            </section>

            <section slot=b class="split-section">
                <div class="editor-container">
                    <CodeMirror bind:this={editor} />
                </div>
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
        height: 4em;
        min-height: 4em;
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

    .editor-container {
        height: 100%;
    }

    .split-pane {
        max-height: 100%;
        flex-grow: 1;
    }

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
