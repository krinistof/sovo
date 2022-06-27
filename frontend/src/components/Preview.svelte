<script lang="ts">
    import {getMeta, getSession} from "../App.svelte"
    import type {SongMeta} from "../types/song.type"
    import {refreshPartyState} from "./Queue.svelte"
    import {searchTerm} from "../stores/searchTerm"
    import Song from "./Song.svelte"
    export let meta: SongMeta
    let songid = meta._id

    function propose() {
        searchTerm.set("")
        let partyid = localStorage.getItem("partyid")
        getSession()
        let sessionid = localStorage.getItem("session")
        getMeta().then(meta => {
            let backend = meta.backend
            fetch(`${backend}/propose?partyid=${partyid}&sessionid=${sessionid}&songid=${songid}`)
                .then(refreshPartyState)
        })
    }
</script>

<div on:dblclick={propose}
    class="w-100 text-center flex shadow bg-slate-200 dark:bg-slate-900 dark:text-white rounded-md p-1 m-2">
    <div class="m-auto" >
        <Song {songid} />
    </div>
</div>
