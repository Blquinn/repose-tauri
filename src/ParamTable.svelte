<script lang="ts">
    import type {ParamTableRow} from "./types";

    export let show: boolean = true;
    export let rows: ParamTableRow[];

    const _r: ParamTableRow = {key: '', value: '', description: ''};
    let cols: string[] = Object.keys(_r);

    function removeRowIfEmpty(row: ParamTableRow) {
        if (row.key === '') { // Delete row if key is empty.
            rows.splice(rows.indexOf(row), 1);
        }
    }

    function onCellChanged(i: number, col: string) {
        if (col !== 'key') { return; }

        if (i !== rows.length-1) {
            removeRowIfEmpty(rows[i]);
            return;
        }

        rows.push({key: '', value: '', description: ''});
    }
</script>

<main style={show ? '' : 'display: none;'}>
    <table class="table">
        <tr>
            <th>Key</th>
            <th>Value</th>
            <th>Description</th>
        </tr>
        {#each rows as row, i}
            <tr>
                {#each cols as col}
                    <td>
                        <input type="text" class="input" bind:value={row[col]}
                               autocomplete="off"
                               on:change={() => onCellChanged(i, col)}
                        >
                    </td>
                {/each}
            </tr>
        {/each}
    </table>
</main>

<style>
    main {
        width: 100%;
        height: 100%;
        overflow-y: auto;
        flex-grow: 1;
    }
    .table {
        width: 100%;
        overflow-y: scroll;
    }
    td {
        cursor: text;
        width: calc(100% / 3);
    }
    tr {
        height: 38px;
    }
    .input {
        max-width: 100%;
        max-height: 100%;
    }
    .input {
        border: none;
        height: 30px;
    }
</style>
