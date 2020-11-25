<script lang="ts">
    import CodeMirror from "./CodeMirror.svelte";
    import ParamTable from "./ParamTable.svelte";
    import {ActiveRequestBodyTab, ActiveRequestEditorTab} from "./types";
    import type {RequestState} from "./types";
    import {activeRequest} from "./state";

    export let show = true;

    let activeTabs = Object.keys(ActiveRequestEditorTab).map(k => [k,ActiveRequestEditorTab[k]]);
    let bodyTabs = Object.keys(ActiveRequestBodyTab).map(k => [k, ActiveRequestBodyTab[k]]);

    let editor: any = null;
    let acHeight: number = 0;

    function onRequestEditorTabClicked(tab: ActiveRequestEditorTab) {
        activeRequest.update(r => {
            if (!r) return null;
            return { ...r, activeRequestEditor: tab } as RequestState;
        });
    }

    function onRequestEditorBodyClicked(tab: ActiveRequestBodyTab) {
        activeRequest.update(r => {
            if (!r) return null;
            return { ...r, activeRequestBody: tab } as RequestState;
        });
    }
</script>

<main style="{show ? '' : 'display: none;'}">
    <div class="tabs">
        <ul>
            {#each activeTabs as [tab, name]}
                <li class={$activeRequest.activeRequestEditor === tab ? 'is-active' : ''}
                    on:click={() => onRequestEditorTabClicked(tab)}
                >
                    <a href="/#">{name}</a>
                </li>
            {/each}
        </ul>
    </div>

    <div class="active-component-container">
        <ParamTable show={$activeRequest.activeRequestEditor === ActiveRequestEditorTab.Params} />
        <ParamTable show={$activeRequest.activeRequestEditor === ActiveRequestEditorTab.Headers} />
        <div style="{$activeRequest.activeRequestEditor === ActiveRequestEditorTab.Body ? '' : 'display: none;'}"
             class="editor-container"
        >
            <CodeMirror bind:this={editor} flex={true} />

            <div class="tabs tabs-up">
                <ul>
                    {#each bodyTabs as [tab, name]}
                        <li class={$activeRequest.activeRequestBody === tab ? 'is-active' : ''}
                            on:click={() => onRequestEditorBodyClicked(tab)}
                        >
                            <a href="/#">{name}</a>
                        </li>
                    {/each}
                </ul>
            </div>
        </div>
    </div>
</main>

<style>
    main {
        flex-grow: 1;
        display: flex;
        flex-direction: column;
    }
    .tabs {
        flex-shrink: 0;
        margin-bottom: 3px;
    }
    .active-component-container {
        flex: 1;
        display: flex;
        flex-direction: column;
        min-height: 0;
    }
    .editor-container {
        flex: 1;
        display: flex;
        flex-direction: column;
    }
</style>
