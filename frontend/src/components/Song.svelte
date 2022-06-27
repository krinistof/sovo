<script lang="ts">
    import type {SongMeta} from "../types/song.type";
    import {getMeta} from "../App.svelte";

    export let songid: String

    let meta: SongMeta = {
        title: "",
        artist: "",
        duration: 0,
        _id: ""
    }
    $: getMeta().then(state => {
        meta = state["songs"][songid];
    })
</script>

<div class="w-100 text-center shadow bg-slate-200 dark:bg-slate-900 dark:text-white rounded-md p-1 m-2">
    <div class="text-3xl font-bold select-none">{meta.title}</div>
    <div class="text-xl bottom font-light select-none">{meta.artist}</div>
</div>
