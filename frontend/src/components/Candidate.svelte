<script lang="ts">
    import type {CandidateMeta} from "../types/candidate.type"
    import {refreshPartyState} from "./Queue.svelte"
    import Song from "./Song.svelte"
    import {getMeta} from "../App.svelte";
    export let meta: CandidateMeta
    function vote(isLike: boolean){
        let partyid = localStorage.getItem("partyid")
        let sessionid = localStorage.getItem("session")
        getMeta().then(metajson => {
            let backend = metajson.backend
            fetch(`${backend}/vote?partyid=${partyid}&sessionid=${sessionid}&songid=${meta.songid["$oid"]}&is_like=${isLike}`)
                .then(refreshPartyState)
        })

    }
    function like(){
        vote(true);
    }
    function dislike(){
        vote(false);
    }
</script>


<div class:liked={meta.likeStatus === 1}
     class:disliked={meta.likeStatus === -1}
     class="w-100 text-center flex shadow bg-slate-200 dark:bg-slate-900 dark:text-white rounded-md p-1 m-2">
    <div class="element">{meta.rank}</div>
    <div class="element" on:click={like}>+</div>
    <div class="w-1/2 p-1 h-40">
        <Song songid={meta.songid["$oid"]} />
    </div>
    <div class="element" on:click={dislike}>-</div>
</div>

<style>
    .disliked {
        @apply bg-red-700
    }
    .liked {
       @apply bg-green-700
    }
    .element {
       @apply text-4xl m-auto w-1/6
    }
</style>
