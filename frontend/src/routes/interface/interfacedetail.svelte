<script lang="ts">
    import InterfaceInfo from "./+page.svelte";
    import Signal from "./+page.svelte";
    export let interfaceInfo: InterfaceInfo;
    let data: Signal;
    $: data = interfaceInfo.signal;
</script>


<div class="card h-16 flex">
    <div class="pl-5 w-full flex items-center variant-soft-secondary">
        <strong>{interfaceInfo.name}</strong>
    </div>
    <div class="flex justify-end w-fit">
        <button class="btn-md variant-filled-primary"> Edit</button>
    </div>
</div>
<div class="card pl-5 py-4 grid grid-cols-2">
    <p>Address:</p>
    <p>{interfaceInfo.address}</p>
    <p>Uuid:</p>
    <p>{interfaceInfo.uuid}</p>
    <p>Type:</p>
    <p>{interfaceInfo.type}</p>
</div>
<hr class="!border-t-4" />
<div class="flex flex-col">
    {#each Object.entries(data) as [type, info] }
        <div class="pl-5 flex flex-col w-full card">
            <div class="grid grid-cols-2 signal py-2">
                <h3 class="col-span-2" >{type}</h3>
                <p>Name:</p> 
                <p>{info.name}</p>
                <p>Unit of Measurement:</p> 
                <p>{info.uom}</p> 
                <p>Symbol:</p> 
                <p>{info.uom_display}</p> 
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
