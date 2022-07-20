<script lang="ts" context="module">
    let metadata = null

    export function getMeta() {
        console.log(metadata)
        if(metadata === null){
            let cdn: String = document
               .querySelector("meta[name=cdn-address]").content;
            return fetch(`${cdn}/songs/meta.json`)
                .then(resp => resp.json())
                .then(data => metadata = data)
        }
        return Promise.resolve(metadata)
    }
    export function getSession(){
        let trySession = localStorage.getItem("session")
        if(trySession === null) {
            let backend: String = document
                .querySelector("meta[name=backend-address]").content;
            return fetch(`${backend}/auth/get-session`)
                .then(resp => resp.json())
                .then(result => localStorage.setItem("session", result))
        } else
            return Promise.resolve(trySession)
    }
</script>
<script lang="ts">
    import Create from './components/Create.svelte'
    import Login from "./components/Login.svelte"
    import Guest from './components/Guest.svelte'
    import Host from './components/Host.svelte'
    import {Route, router} from 'tinro'
    import {onMount} from "svelte"

    router.mode.hash()
    function createParty() {
        router.goto("/create")
    }

    function logIn() {
        router.goto("/login")
    }
</script>

<main>
    <Route path="/">
        <div class="text-4xl text-center h-auto py-auto">SOVO</div>
        <div class="text-xl italic text-center h-auto py-auto">song voter</div>
        <div class="flex space-x-auto">
            <button on:click={createParty} class="m-auto font-bold px-1 py-2 rounded hover:bg-green-700 bg-green-500">create new party</button>
            <button on:click={logIn} class="m-auto font-bold px-1 py-2 rounded hover:bg-blue-700 bg-blue-500">login as host</button>
        </div>
    </Route>
    <Route path="/create">
        <Create/>
    </Route>
    <Route path="/login">
        <Login/>
    </Route>
    <Route path="/host/:partyid" let:meta>
        <Host partyid={meta.params.partyid}/>
    </Route>
    <Route path="/r/:partyid" let:meta>
        <Guest partyid={meta.params.partyid}/>    
    </Route>
</main>

<style lang="postcss" global>
    @tailwind base;
    @tailwind components;
    @tailwind utilities;
</style>
