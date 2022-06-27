import {writable} from 'svelte/store'
export const player = writable({
            player: null,
            inc_interval : null,
            dec_interval : null,
            timeout : null
        })
