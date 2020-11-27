<script lang="ts">
    import {clickOutside} from "./click_outside";
    import CodeMirror from "./CodeMirror.svelte";
    import ParamTable from "./ParamTable.svelte";
    import type {RequestBodyRaw, RequestState, RequestTypes} from "./types";
    import {ActiveRequestEditorTab, requestTypeKeys} from "./types";
    import {activeRequest} from "./state";
    import {onMount} from "svelte";
    import {editorModes} from "./consts";

    export let show = true;

    const activeTabs = Object.keys(ActiveRequestEditorTab).map(k => [k, ActiveRequestEditorTab[k]]);

    let editor: any = null;
    let acHeight: number = 0;

    function onRequestEditorTabClicked(tab: ActiveRequestEditorTab) {
        activeRequest.update(r => {
            if (!r) return null;
            return {...r, activeRequestEditor: tab} as RequestState;
        });
    }

    function onRequestEditorBodyClicked(type: string) {
        activeRequest.update(r => {
            if (!r) return null;
            return {...r, activeRequestBody: type} as RequestState;
        });
    }

    let lastMode: string | undefined = undefined;
    let lastRequestId: string | undefined = undefined;
    const emptyBodyMsg = ''

    let isRawBodyTypeActive = false;
    let modes: string[] = Object.keys(editorModes);

    activeRequest.subscribe(req => {
        if (!editor) return;

        if (!req) {
            editor.set(emptyBodyMsg, 'plain');
            return;
        }

        // TODO: Store request mode in state.

        const rawBody = req.requestBodies['raw'] as RequestBodyRaw;

        if (req.id !== lastRequestId || rawBody.mode != lastMode) {
            const body = rawBody.body !== '' ? rawBody.body : emptyBodyMsg;
            editor.set(body, rawBody.mode);
            lastRequestId = req.id;
            lastMode = rawBody.mode
        }
    });

    function onEditorChange() {
        const val = editor.getValue();

        activeRequest.update(req => {
            if (!req) return;

            const rawBody = req.requestBodies['raw'] as RequestBodyRaw;

            const requestBodies: Record<string, RequestTypes> = {
                ...req.requestBodies,
                'raw': { ...rawBody, body: val }
            };

            return { ...req, requestBodies };
        });
    }

    function onRequestModeChanged(mode: string) {
        activeRequest.update(req => {
            const rawBody = req.requestBodies['raw'] as RequestBodyRaw;

            const requestBodies: Record<string, RequestTypes> = {
                ...req.requestBodies,
                'raw': { ...rawBody, mode: mode }
            };

            return { ...req, requestBodies }
        });

        isRawBodyTypeActive = false;
    }

    onMount(() => {
        editor.set(emptyBodyMsg, 'plain');
    });
</script>

<main class:is-hidden={!show}>
    <div class="tabs">
        <ul>
            {#each activeTabs as [tab, name]}
                <li class:is-active={$activeRequest.activeRequestEditor === tab}
                    on:click={() => onRequestEditorTabClicked(tab)}
                >
                    <a href="/#">{name}</a>
                </li>
            {/each}
        </ul>
    </div>

    <div class="active-component-container">
        <ParamTable rows={$activeRequest.params}
                    show={$activeRequest.activeRequestEditor === ActiveRequestEditorTab.Params}/>
        <ParamTable rows={$activeRequest.headers}
                    show={$activeRequest.activeRequestEditor === ActiveRequestEditorTab.Headers}/>

        <div class="editor-container"
             class:is-hidden={$activeRequest.activeRequestEditor !== ActiveRequestEditorTab.Body}
        >
            <div class="no-request-body-container"
                 class:is-hidden={$activeRequest.activeRequestBody !== 'none'}
            >
                <h4>No Request Body</h4>
            </div>

            <ParamTable rows={
                        ($activeRequest.activeRequestBody === 'form-data' || $activeRequest.activeRequestBody === 'x-www-form-urlencoded')
                            ? $activeRequest.requestBodies[$activeRequest.activeRequestBody].rows : []}
                        show={$activeRequest.activeRequestBody === 'form-data' || $activeRequest.activeRequestBody === 'x-www-form-urlencoded'}
            />

            <CodeMirror bind:this={editor} flex={true}
                        on:blur={onEditorChange}
                        show={$activeRequest.activeRequestBody === 'raw'} />

            <div class="binary-body" class:is-hidden={$activeRequest.activeRequestBody !== 'binary'}>
                <div class="binary-body-row">
                    <input class="button is-primary" on:click={() => alert('clicked file diag')} type="button"
                           value="Select File...">
                    <p>{
                        $activeRequest.requestBodies['binary'].filePath !== ''
                            ? $activeRequest.requestBodies['binary'].filePath
                            : 'No file selected'
                    }</p>
                </div>
            </div>

            <div class="tabs tabs-up" class:overflow-visible={isRawBodyTypeActive}>
                <ul>
                    {#each requestTypeKeys as type}
                        <li class="body-type" class:is-active={$activeRequest.activeRequestBody === type}>
                            <a href="/#" on:click={() => onRequestEditorBodyClicked(type)}>
                                <span>{type}</span>
                                {#if type === 'raw'}
                                    <span>
                                        <div class="dropdown is-up is-active" class:is-active={isRawBodyTypeActive} use:clickOutside on:click_outside={() => isRawBodyTypeActive = false}>
                                            <div class="dropdown-trigger" on:click={() => isRawBodyTypeActive = true}>
                                                <span class="icon is-small body-type-icon">
                                                    <i class="fas fa-angle-up" aria-hidden="true"></i>
                                                </span>
                                            </div>
                                            <div class="dropdown-menu" id="dropdown-menu7" role="menu">
                                                <div class="dropdown-content">
                                                    {#each modes as mode}
                                                        <p class="dropdown-item"
                                                           class:is-active={$activeRequest.requestBodies['raw'].mode === mode}
                                                           on:click={() => onRequestModeChanged(mode)}
                                                        >{mode}</p>
                                                    {/each}
                                                </div>
                                            </div>
                                        </div>
                                    </span>
                                {/if}
                            </a>
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

    .no-request-body-container {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .no-request-container > h4 {
        text-align: center;
    }

    .binary-body {
        flex: 1;
    }

    .binary-body-row {
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .binary-body-row > p {
        margin-left: 5px;
    }

    li.body-type {
        display: flex;
        flex-direction: row;
    }

    .overflow-visible {
        overflow-x: visible;
        overflow-y: visible;
    }
</style>
