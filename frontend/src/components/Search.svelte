<script lang="ts">
    import type {SongMeta} from "../types/song.type"
    import Preview from './Preview.svelte'
    import {searchTerm} from "../stores/searchTerm"
    import {getMeta} from "../App.svelte";
    let search = []

    searchTerm.subscribe(query => {
        query = query.toLowerCase()

        getMeta().then(meta => {
            let songmeta = meta.songs
            search = []
            for(const elem in songmeta) {
                let song = songmeta[elem]
                if(song.title.concat(song.artist).toLowerCase().includes(query)) {
                    song._id = elem
                    search = [...search, song]
                }
            }
        })
    })
</script>
<div class="bg-slate-300 dark:bg-slate-700 container rounded-md p-1">
    <div class="text-3xl text-center m-7">
        double tap on the song to propose it!🎵
    </div>
    {#each search as song (song._id)}
        <Preview meta={song}/>
    {/each}
</div>

<style>
</style>
