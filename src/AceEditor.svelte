<script lang="ts">
    import * as ace from 'brace';
    import {onMount, tick} from "svelte";
    // import { watchResize } from "svelte-watch-resize";
    // require('brace/mode/javascript');
    import('brace/mode/javascript');

    let editorElement: any;

    let editor: ace.Editor;

    let height = undefined;
    let width = undefined;

    export let lang = 'javascript'

    onMount(() => {
        editor = ace.edit(editorElement);
        // editor.getSession().setMode('ace/mode/'+lang);
        // editor.getSession().setMode('ace/mode/'+lang);
        // editor.set
    })

    /*
    $: if (height !== undefined && width !== undefined) { onDivResize(); }

    function onDivResize() {
        console.log('Resize')
        if (editor) tick().then(() => editor.resize());
    }
    */
</script>

<div bind:this={editorElement} class="editor"></div>

<style>
    .editor {
        height: 100%;
        width: 100%;
    }
</style>
