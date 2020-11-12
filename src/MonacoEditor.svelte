<script lang="ts">
    import { onMount } from 'svelte';
    import * as monaco from 'monaco-editor';

    var containerElt: any;

    onMount(() => {
        monaco.editor.create(containerElt, {
            value: ['function x() {', '\tconsole.log("Hello world!");', '}'].join(
                '\n',
            ),
            language: 'javascript',
            fontFamily: 'Roboto_Mono',
        })
    })
</script>

<style>
    .editor-container {
        width: 100%;
        height: 100%;
        font-family: var(--font-mono);
    }

    .monaco-editor {
        font-family: var(--font-mono);
    }
</style>

<div bind:this={containerElt} class="editor-container"></div>
