<script lang="ts">

   import type {PartyState} from "../types/partyState.type"
   import {refreshPartyState} from "./Queue.svelte";
   import {partyState} from "../stores/partyState";
   import type {SongMeta} from "../types/song.type";
   import {getMeta} from "../App.svelte";

   export let partyid
   localStorage.setItem("partyid", partyid);
   const url = `${location.origin}/#r/${partyid}`
   let player = new Audio();


   function stopEverything(){
       player.pause()
       let id = window.setTimeout(function() {}, 0);

       while (id--) {
           window.clearTimeout(id); // will do nothing if no timeout with id is present
       }
   }
   function nextSong(){
       stopEverything()
       if($partyState.queue.length == 0){
           setTimeout(refreshPartyState, 15000)
           return
       }
       getMeta()
           .then(meta => {
               let backend = meta.backend
               let password = localStorage.getItem("password")
               fetch(`${backend}/next?partyid=${partyid}&password=${password}`)
                   .then(resp => resp.json())
                   .then(data => {
                       refreshPartyState()
                       setTimeout(() => {
                           if(!data){
                               setTimeout(nextSong, 10000)
                               return
                           }
                           playCurrent()
                       }, 500)
                   })
           })

   }

   function playCurrent(){
       if(!player.paused)
           return
       let songid = $partyState.currentSong["$oid"]
       player = new Audio(`songs/${songid}`)
       player.play()
       getMeta()
           .then(meta => {
               let song: SongMeta = meta["songs"][songid]
               console.log(song)
               setTimeout(nextSong, song.duration*1000)
           })
   }

   function toggleParty() {
       getMeta()
           .then(meta => {
               let backend = meta.backend
               let password = localStorage.getItem("password")
               fetch(`${backend}/toggle?partyid=${partyid}&password=${password}`)
                   .then(() => setTimeout(refreshPartyState, 500))
           })
   }

   refreshPartyState()
   let live = false;
   partyState.subscribe((party: PartyState) => {
       if(!party)
           return
       console.table(party)
       live = party.isLive
       if(live){
           if(party.currentSong)
               playCurrent()
           else
               nextSong()
       } else {
           stopEverything()
           setTimeout(refreshPartyState, 15000)
       }
   });
   //TODO: start and stop party, moderate
</script>

<div class="text-3xl text-center p-5 m-4">
    share the following url:
    <div class="bg-blue-500 max-w-md m-auto p-2">
        <a href={url} class="text-white">{url}</a>
    </div>

    <div class="max-w-md m-auto p-2">
        party is
        {#if !live}
            <span>not</span>
        {/if}
        live
    </div>

    <button on:click={toggleParty}>toggle party</button>
    {#if live}
        <button on:click={nextSong}>skip song</button>
    {/if}
</div>
