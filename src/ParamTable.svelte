<script lang="ts">
    export let show: boolean = true;

    interface ParamRow {
        key: string;
        value: string;
        description: string;
    }

    interface CellKey {
        row: number;
        col: string;
    }

    let activeCell: CellKey | null = null;

    let cols = ['key', 'value', 'description'];

    let myRows: ParamRow[] = [
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
        {key: 'bar', value: 'baz', description: ''},
    ]

    function removeRowIfEmpty(row: ParamRow) {
        if (row.key === '') { // Delete row if key is empty.
            myRows.splice(myRows.indexOf(row), 1);
        }
    }

    function onInputKeyPress(e: KeyboardEvent) {
        if (activeCell === null) return;

        if (e.key === 'Enter' || e.key === 'Tab')
            removeRowIfEmpty(myRows[activeCell.row]);

        if (e.key === 'Enter') activeCell = null;

        // Find next cell and mark it for editing on tab.
        if (e.key === 'Tab') {
            e.preventDefault();

            const colIdx = cols.indexOf(activeCell.col);
            const nextColIdx = (colIdx + 1) % cols.length;

            if (nextColIdx > 0) {
                activeCell = {
                    ...activeCell,
                    col: cols[nextColIdx],
                };
                return;
            }

            if (activeCell.row + 1 >= myRows.length) {
                activeCell = null;
                return;
            }

            activeCell = {
                ...activeCell,
                row: activeCell.row + 1,
                col: cols[nextColIdx],
            };
        }
    }
</script>

<main style={show ? '' : 'display: none;'}>
    <table class="table">
        <tr>
            <th>Key</th>
            <th>Value</th>
            <th>Description</th>
        </tr>
        {#each myRows as row, i}
            <tr>
                {#each cols as col}
                    {#if activeCell && i === activeCell.row && col === activeCell.col}
                        <td>
                            <input autofocus type="text" class="input is-small"
                                   bind:value={row[col]}
                                   on:blur={() => {
                                       activeCell = null;
                                       removeRowIfEmpty(row);
                                   }}
                                   on:keydown={onInputKeyPress}
                            >
                        </td>
                    {:else}
                        <td on:click={() => activeCell = {row: i, col: col}}>{row[col]}</td>
                    {/if}
                {/each}
            </tr>
        {/each}
        <!-- Last row is a placeholder row. -->
        {#if myRows[myRows.length-1].key !== ''}
            <tr>
                {#each cols as col}
                    <td on:click={() => {
                        myRows.push({key: '', value: '', description: ''});
                        activeCell = {row: myRows.length-1, col: col}
                    }}
                    ></td>
                {/each}
            </tr>
        {/if}
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
</style>
