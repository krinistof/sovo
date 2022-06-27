<script lang="ts">
    import {SHA256} from "crypto-js"
    import {router} from 'tinro';
    import {getMeta} from "../App.svelte";
    import {saltHash} from "./Login.svelte";
    let partyid
    let password
    function createParty() {
        getMeta()
            .then(meta => {
                let backend = meta.backend
                saltHash(password)
                password = localStorage.getItem("password")
                fetch(`${backend}/create-party?partyid=${partyid}&password=${password}`)
                    .then(() => {router.goto(`/host/${partyid}`)})

            })
    }
</script>

<div class="grid place-items-center h-screen">
    <div>
        <label for="partyid" >short name of party</label>
        <input bind:value={partyid}
               type="text"
               id="partyid"
               placeholder="bestpartyever"
               class="text-black">
    </div>
    <div>
        <label for="password" >password:</label>
        <input bind:value={password}
               type="password"
               id="password"
               class="text-black">
    </div>
    <div class="flex">
       <button on:click={createParty}>create party</button>
    </div>
</div>
