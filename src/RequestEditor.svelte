<script lang="ts">
    import CodeMirror from "./CodeMirror.svelte";
    import ParamTable from "./ParamTable.svelte";

    export let show = true;

    enum ActiveTab {
        Params = 'Params',
        Headers = 'Headers',
        Body = 'Body',
    }

    let activeTab: ActiveTab = ActiveTab.Params;

    let editor: any = null;

    let acHeight: number = 0;
</script>

<main style="{show ? '' : 'display: none;'}">
    <div class="tabs">
        <ul>
            <li class={activeTab === ActiveTab.Params ? 'is-active' : ''}
                on:click={() => activeTab = ActiveTab.Params}
            ><a href="/#">Params</a></li>
            <li class={activeTab === ActiveTab.Headers ? 'is-active' : ''}
                on:click={() => activeTab = ActiveTab.Headers}
            ><a href="/#">Headers</a></li>
            <li class={activeTab === ActiveTab.Body ? 'is-active' : ''}
                on:click={() => activeTab = ActiveTab.Body}
            ><a href="/#">Body</a></li>
        </ul>
    </div>

    <div class="active-component-container">
        <ParamTable show={activeTab === ActiveTab.Params} />
        <ParamTable show={activeTab === ActiveTab.Headers} />
        <CodeMirror show={activeTab === ActiveTab.Body} bind:this={editor} flex={true} />
    </div>
</main>

<style>
    main {
        flex-grow: 1;
        display: flex;
        flex-direction: column;
    }
    .request-details-bar > * {
        margin-left: 2px;
        margin-right: 2px;
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
</style>
