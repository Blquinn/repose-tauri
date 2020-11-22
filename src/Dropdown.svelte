<script lang="ts">
    import {clickOutside} from "./click_outside";

    export let options: string[];
    export let activeValue: string | null = options.length > 0 ? options[0] : null;

    let active = false;
</script>

<div class="dropdown" class:is-active={active} use:clickOutside on:click_outside={() => active = false}>
    <div class="dropdown-trigger">
        <button class="button" aria-haspopup="true" aria-controls="dropdown-menu" on:click={() => active = !active}>
            <span class="value">{activeValue ?? ''}</span>
            <span class="icon is-small">
            <i class="fas fa-angle-down" aria-hidden="true"></i>
          </span>
        </button>
    </div>
    <div class="dropdown-menu" id="dropdown-menu" role="menu">
        <div class="dropdown-content">
            {#each options as option}
                <a href="#"
                   class="dropdown-item"
                   on:click={() => {
                       activeValue = option;
                       active = false;
                   }}
                >{option}</a>
            {/each}
        </div>
    </div>
</div>

<style>
    .value {
        min-width: 5em;
    }
</style>
