<script lang="ts">

    import InterfaceDetail from './interfacedetail.svelte'; 

    type InterfaceInfo = {
        name: string;
        address: string;
        uuid: string;
        type: string;
        signal: Interface;
    }

    type Interface = {
        [key:string]: Signal
    }

    type Signal = {
        name: string;
        uom: string;
        uom_display: string;
    }
    
    const data: InterfaceInfo[] = [
    {name: "Sensor A",
    address: "192.168.0.241:8080",
    uuid: "item-a",
    type: "ShellyV1",
    signal: {
        "emeter_1" : {name: "emeter_1", uom: "watt", uom_display: "W"},
        "emeter_2" : {name: "emeter_2", uom: "watt", uom_display: "W"},
        "emeter_3" : {name: "emeter_3", uom: "watt", uom_display: "W"},
        "emeter_4" : {name: "emeter_4", uom: "watt", uom_display: "W"}
    }},
    {name: "Sensor B",
    address: "192.168.0.245:aslkd",
    uuid: "item-b",
    type: "ShellyV2",
    signal: {
        "temp_100" : {name: "1", uom: "jul", uom_display: "J"},
        "temp_101" : {name: "2", uom: "jul", uom_display: "J"},
        "temp_102" : {name: "3", uom: "jul", uom_display: "J"},
        "temp_103" : {name: "4", uom: "jul", uom_display: "J"},
    }},
    ];

    let selected: InterfaceInfo = data[0];

    function setSelected(sensor: InterfaceInfo) {
        selected = sensor
    }

</script>

<div class="flex flex-row h-full">
    <div class="card variant-filled-surface center w-1/4 p-5">
        <input class="input" type="search" placeholder="Search"/>
        <nav class="list-nav">
            <ul >
                {#each data as sensor}
                    <li>
                        <button class="w-full" on:click={() => setSelected(sensor)}>
                        <span class="flex-left">{sensor.name}</span>
                        <span class="flex-auto">{sensor.address}</span>
                        </button>
                    </li>
                {/each}
            </ul>
        </nav>

    </div>
    <div class="card variant-filled-surface w-full">
        <InterfaceDetail interfaceInfo={selected} />
    </div>
</div>


<style>
</style>
