<script lang="ts">
    import type { PageServerData } from './$types';

    export let data: PageServerData;
</script>

<div>
    {#if data.isAdmin}
        <p>Im {data.id} and i am an admin</p>
    {:else}
        <p>Im {data.id} and i am not an admin</p>
    {/if}

</div>
