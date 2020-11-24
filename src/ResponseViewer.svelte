<script lang="ts">
    import {activeRequest} from "./state";
    import CodeMirror from "./CodeMirror.svelte";

    export let show = true;

    let editor: CodeMirror;

    enum DisplaySection { Headers, ResponseBody }
    let active = DisplaySection.ResponseBody;

    let lastBody: string | undefined = undefined;

    const mimeModeMap = {
        'application/json': 'json',
        'text/javascript': 'js',
        'text/xml': 'xml',
        'application/xml': 'xml',
        'text/html': 'html',
    }

    $: if (editor && $activeRequest && $activeRequest.lastResponse && lastBody !== $activeRequest.lastResponse?.body) {
        const lastResponse = $activeRequest.lastResponse;

        let mode = 'plain';
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
        }

        lastBody = lastResponse?.body;

        editor.set(lastBody ?? 'Empty response.', mode);
    }
</script>

<main style="{show ? '' : 'display: none;'}">
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

    <div class="headers" style={active === DisplaySection.Headers ? '' : 'display: none;'}>
        {#if $activeRequest.lastResponse}
        {#each $activeRequest.lastResponse.headers.all() as header}
            <p><b>{header[0]}:</b> {header[1]}</p>
        {/each}
        {/if}
    </div>
    <CodeMirror class="editor" bind:this={editor} show={active === DisplaySection.ResponseBody} readonly={true} />
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
    }
    .tabs {
        margin-bottom: 3px;
    }
</style>
