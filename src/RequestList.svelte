<script lang="ts">
    import List, {Item, Text} from '@smui/list';
    import Textfield, {Input, Textarea} from '@smui/textfield';
    import Icon from '@smui/textfield/icon/index';

    import type {RootState} from "./state";
    import {writable} from "svelte/store";

    export let state: writable<RootState>;
    let searchText = '';

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
    <Textfield class="shaped-outlined" variant="outlined" withTrailingIcon bind:value={searchText}
               label="Filter" input$aria-controls="helper-text-shaped-outlined-c"
               input$aria-describedby="helper-text-shaped-outlined-c">
<!--        <Icon class="material-icons">delete</Icon>-->
    </Textfield>
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
