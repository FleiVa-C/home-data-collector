<script lang="ts">
    import type { PageData } from './$types';
    import type { InterfaceData } from '$lib/types';
    import InterfaceDetail from './interfacedetail.svelte'; 

    export let data: PageData;

    let selected: InterfaceData = data.interfaces[0];

    function setSelected(sensor: InterfaceData) {
        selected = sensor
    };
</script>

<div class="flex flex-row h-full">
    <div class="variant-filled-surface center w-1/4 p-5">
        <input class="input" type="search" placeholder="Search"/>
        <nav class="list-nav">
            <ul >
                {#each data.interfaces as sensor}
                    <li>
                        <button class="w-full" on:click={() => setSelected(sensor)}>
                        <span class="flex-left">{sensor.name}</span>
                        <span class="flex-auto">{sensor.url}</span>
                        </button>
                    </li>
                {/each}
            </ul>
        </nav>

    </div>
    <div class="variant-filled-surface w-full">
        <InterfaceDetail interfaceInfo={selected} />
    </div>
</div>


<style>
</style>
