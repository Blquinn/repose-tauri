<script lang="ts">
    import {activeRequest} from "./state";
    import CodeMirror from "./CodeMirror.svelte";
    import {onMount} from "svelte";

    export let show = true;

    let editor: CodeMirror;

    enum DisplaySection { Headers, ResponseBody }
    let active = DisplaySection.ResponseBody;

    const mimeModeMap = {
        'application/json': 'json',
        'text/javascript': 'js',
        'text/xml': 'xml',
        'application/xml': 'xml',
        'text/html': 'html',
    }

    let lastBody: string | undefined = undefined;
    const emptyBodyMsg = 'Empty Response'

    activeRequest.subscribe(req => {
        if (!editor) return;

        if (!req) {
            editor.set(emptyBodyMsg, 'plain');
            return;
        }

        // TODO: Use request ids to track changes.
        let mode = 'plain';
        const lastResponse = req.lastResponse;
        if (lastResponse) {
            const ct = lastResponse.headers.get('content-type');
            if (ct) {
                const chunks = ct.split(';');
                if (chunks.length > 0) {
                    const ctt = chunks[0];

                    const m = mimeModeMap[ctt] ?? null;
                    if (m) mode = m;
                }
            }

            lastBody = lastResponse.body;
        } else {
            lastBody = undefined;
        }

        editor.set(lastBody ?? emptyBodyMsg, mode);
    });

    onMount(() => {
        editor.set(emptyBodyMsg, 'plain');
    });
</script>

<main class:is-hidden={!show}>
    <div class="tabs">
        <ul>
            <li  class:is-active={active === DisplaySection.Headers}
                 on:click={() => active = DisplaySection.Headers}
            ><a href="/#">Headers</a></li>

            <li class:is-active={active === DisplaySection.ResponseBody}
                on:click={() => active = DisplaySection.ResponseBody}
            ><a href="/#">Body</a></li>
        </ul>
    </div>

    <div class="response-container">
        <div class="loader-wrapper" class:is-active={$activeRequest.isLoading}>
            <div class="loader" class:is-loading={$activeRequest.isLoading}></div>
        </div>

        <div class="headers" class:is-hidden={active !== DisplaySection.Headers}>
            {#if $activeRequest.lastResponse}
            {#each $activeRequest.lastResponse.headers.all() as header}
                <p><b>{header[0]}:</b> {header[1]}</p>
            {/each}
            {/if}
        </div>
        <CodeMirror class="editor" bind:this={editor} show={active === DisplaySection.ResponseBody} readonly={true} />
    </div>
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
    .editor, .headers {
        flex: 1;
        min-height: 0;
    }
    .headers {
        overflow-wrap: break-word;
        overflow-y: scroll;
        padding: 5px;
    }
    .tabs {
        margin-bottom: 3px;
    }
    .response-container {
        flex: 1;
        display: flex;
        flex-direction: column;
    }
    .response-container > * {
        flex: 1;
        flex-shrink: 0;
    }
    .loader-wrapper {
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        width: 100%;
        background: #fff;
        opacity: 0;
        z-index: -1;
        transition: opacity .3s;
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 6px;
    }
    .loader-wrapper .loader {
        height: 80px;
        width: 80px;
    }
    .loader-wrapper.is-active {
         opacity: 1;
         z-index: 4;
    }
</style>
