<script lang="ts">
    import CodeMirror from "./CodeMirror.svelte";

    export let show = true;

    let editor: any = null;
</script>

<main style="{show ? '' : 'display: none;'}">
    <div class="tabs">
        <ul>
            <li><a href="#">Params</a></li>
            <li><a href="#">Headers</a></li>
            <li class="is-active"><a href="#">Body</a></li>
        </ul>
    </div>

    <CodeMirror bind:this={editor} />
</main>

<style>
    main {
        height: 100%;
    }
</style>
