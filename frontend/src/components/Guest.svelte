<script lang="ts">
    import {searchTerm} from "../stores/searchTerm"
    import {partyState} from "../stores/partyState"
    import Search from './Search.svelte'
    import Queue from './Queue.svelte'
    import Song from "./Song.svelte"

    export let partyid

    localStorage.setItem("partyid", partyid);
    let songid = null
    partyState.subscribe(state => {
        if(state && state.currentSong)
            songid = state.currentSong["$oid"]
    })
</script>

{#if songid}
    <div class="bg-slate-300 dark:bg-slate-700 container rounded-md p-2 my-4">
        <div class="text-center text-4xl p-1">currently playing:</div>
        <Song songid={songid}/>
    </div>
{/if}

<Queue/>

<div>
    <div class="text-center text-3xl p-5">want to propose a song?</div>

    <div class="w-max my-7 m-auto text-black">
        <input class="w-max p-1" type="text" placeholder="type artist or title.." bind:value={$searchTerm}/>
    </div>
</div>
{#if $searchTerm !== ""}
    <Search/>
{/if}
