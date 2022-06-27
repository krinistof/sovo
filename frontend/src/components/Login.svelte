<script lang="ts" context="module">
    import {SHA256} from "crypto-js"
    import {router} from 'tinro';
    export function saltHash(plaintext: String){
        let salted = SHA256("sovo"+plaintext).toString()
        localStorage.setItem("password", salted)
    }
</script>
<script lang="ts">
    let partyid
    let password
    function login() {
        localStorage.setItem("partyid", partyid)
        saltHash(password)
        router.goto(`/host/${partyid}`)
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
        <button on:click={login}>login</button>
    </div>
</div>
