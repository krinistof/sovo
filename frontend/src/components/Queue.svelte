<script lang="ts" context="module">
    import {partyState} from "../stores/partyState"
    import {getMeta, getSession} from "../App.svelte";
    export async function refreshPartyState() {
        let partyid = localStorage.getItem("partyid")
        getSession()
        let sessionid = localStorage.getItem("session")
        let backend: String = document
            .querySelector("meta[name=backend-address]").content;
        fetch(`${backend}/queue?partyid=${partyid}&sessionid=${sessionid}`)
            .then(resp => resp.json())
            .then(result => {
                partyState.set(result)
            })
    }
</script>

<script lang="ts">
    import Candidate from './Candidate.svelte'
    import {onMount} from 'svelte'

    onMount(refreshPartyState)
    setInterval(refreshPartyState, 4000)

    let queue = []
    partyState.subscribe(obj => {
        if(obj != null)
            queue = obj["queue"]
    });
</script>

<div class="bg-slate-300 dark:bg-slate-700 container rounded-md p-1">
    {#if queue && queue.length !== 0}
        {#each queue as song (song.songid)}
            <Candidate meta={song}/>
        {/each}
    {:else }
        <div class="text-3xl text-center p-3">
            no one was brave enough to ask for a song.
        </div>
        <div class="text-3xl text-center p-3">
            be the first one! 😎
        </div>
    {/if}
</div>

