<script lang="ts">
    import List, {Item, Text} from '@smui/list';
    import type {RootState} from "./state";
    import {writable} from "svelte/store";

    export let state: writable<RootState>;

    interface Request {
        text: string;
    }

    let requests: Request[] = [
        {text: 'Gray Kittens'},
        {text: 'A Space Rocket'},
        {text: '100 Pounds of Gravel'},
        {text: 'All of the Shrimp'},
        {text: 'A Planet with a Mall'},
        // {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'},
        // {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'}, {text: 'A Planet with a Mall'},
    ]
</script>

<main>
    <List class="request-list">
        {#each requests as request}
            <Item href="javascript:void(0)" on:click={() => {
                state.update(current => { return {...current, clicked: request.text}});
                localStorage.setItem('clicked', request.text)
            }}>
                <Text>{ request.text }</Text>
            </Item>
        {/each}
    </List>
</main>

<style>
    main {
        /*overflow-y: scroll;*/
        /*height: 100%;*/
        /*height: 100vh;*/
    }
</style>
