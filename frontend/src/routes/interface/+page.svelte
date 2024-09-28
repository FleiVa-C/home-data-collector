<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import Header from '../Header.svelte';
	import DataTable from "./data-table.svelte"
</script>


<body>
	<Header></Header>
	<div class="flex flex-col gap-5 m-5 mt-8">
		<div>
			<h1 class="text-4xl">Interfaces</h1>
			<p class="my-2 text-muted-foreground">Overview of the registered qcquisition interfaces.</p>
		</div>
		<div class="w-full">
			<div class="mb-4 flex items-center gap-4">
				<Input class="h-8 max-w-lg" placeholder="Search..." type="text" ></Input>
			</div>
			<DataTable/>
		</div>

</body>
