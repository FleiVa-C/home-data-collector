<script lang="ts">
    import type { InterfaceData } from "$lib/types";
    export let interfaceInfo: InterfaceData;
    $: editMode = false;

    const switchMode = () => {
        if (editMode) {
            editMode = false
        }else{
            editMode = true;
        };
    };
</script>


<div class="h-16 flex">
{#if editMode}
    <div class="pl-5 w-full flex items-center variant-soft-secondary">
        <input class="input" bind:value={interfaceInfo.name}/>
    </div>
    <div class="flex justify-end w-fit">
        <button class="btn-md variant-filled-error" on:click={switchMode}> Cancel</button>
        <button class="btn-md variant-filled-success" on:click={switchMode}> Save</button>
    </div>
{:else}
    <div class="pl-5 w-full flex items-center variant-soft-secondary">
        <strong>{interfaceInfo.name}</strong>
    </div>
    <div class="flex justify-end w-fit">
        <button class="btn-md variant-filled-primary" on:click={switchMode}> Edit</button>
    </div>
{/if}
</div>
<div class="card pl-5 py-4 grid grid-cols-2">
    <p>URL:</p>
    {#if editMode}
        <input class="input" bind:value={interfaceInfo.url}/>
    {:else}
        <p>{interfaceInfo.url}</p>
    {/if}
    <p>Uuid:</p>
    <p>{interfaceInfo.uuid}</p>
    <p>Type:</p>
    <p>{interfaceInfo.interface_type}</p>
</div>
<hr class="!border-t-4" />
<div class="flex flex-col">
    {#each Object.entries(interfaceInfo.signals) as [type, info] }
        <div class="pl-5 flex flex-col w-full card">
            <div class="grid grid-cols-2 signal py-2">
                <h3 class="col-span-2" >{type}</h3>
                {#if editMode}
                    <p>Name:</p> 
                    <input class="input" bind:value={info.name}/>
                    <p>Unit of Measurement:</p> 
                    <input class="input" bind:value={info.uom}/>
                    <p>Symbol:</p> 
                    <input class="input" bind:value={info.uom_symbol}/>
                {:else}
                    <p>Name:</p> 
                    <p>{info.name}</p>
                    <p>Unit of Measurement:</p> 
                    <p>{info.uom}</p> 
                    <p>Symbol:</p> 
                    <p>{info.uom_symbol}</p> 
                {/if}
            </div>
        </div>
        <hr class="!border-t-2" />
    {/each}
</div>

<style>
    .grid-cols-2{
        grid-template-columns: 200px 1fr;
    }

</style>
