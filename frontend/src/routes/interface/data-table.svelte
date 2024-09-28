<script lang="ts">
	import { createTable, Render, Subscribe } from "svelte-headless-table";
	import { addSortBy } from "svelte-headless-table/plugins";
	import ArrowUpDown from "lucide-svelte/icons/arrow-up-down";
	import { readable } from "svelte/store";
	import * as  Table from "$lib/components/ui/table";
	import { Button } from "$lib/components/ui/button";

type AcquisitionInterface = {
	uuid: string;
	status: "Active" | "Inactive";
	name: string;
	acquisition_url: string;
};

const data: AcquisitionInterface[] = [
		{
			uuid: "uuid_1",
			status: "Active",
			name: "Interface 1",
			acquisition_url: "url_1"
		},
		{
			uuid: "uuid_2",
			status: "Inactive",
			name: "Interface 2",
			acquisition_url: "url_2"
		}
	];

	const table = createTable(readable(data), {
		sort: addSortBy()
	});

	const columns = table.createColumns([
		table.column({
			header: "Status",
			accessor: "status"
		}),
		table.column({
			header: "Name",
			accessor: "name"
		}),
		table.column({
			header: "Acquisition URL",
			accessor: "acquisition_url",
			plugins: {
				sort: {
				disable: true,
				},
			},
		}),
		table.column({
			accessor: ({ uuid }) => uuid,
			header: "",
		}),
	]);

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs } =
		table.createViewModel(columns)
</script>


<div class="rounded-md border">
	<Table.Root {...$tableAttrs}>
		<Table.Header>
			{#each $headerRows as headerRow}
				<Subscribe rowAttrs={headerRow.attrs()}>
					<Table.Row>
						{#each headerRow.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
								<Table.Head {...attrs}>
									{#if cell.id === "status"}
										<Button variant="ghost" class="pl-0 hover:text-foreground-muted hover:bg-muted-background" on:click={props.sort.toggle}>
											<Render of={cell.render()}/>
											<ArrowUpDown class={"ml-2 h-4 w-4 hover:text-foreground"} />
										</Button>
									{:else if cell.id === "name"}
										<Button variant="ghost" class="pl-0 hover:text-foreground-muted hover:bg-muted-background" on:click={props.sort.toggle}>
											<Render of={cell.render()}/>
											<ArrowUpDown class={"ml-2 h-4 w-4 hover:text-foreground" } />
										</Button>
									{:else}
											<Render of={cell.render()}/>
									{/if}
								</Table.Head>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Header>
		<Table.Body {...$tableBodyAttrs}>
			{#each $pageRows as row (row.id)}
				<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
					<Table.Row {...rowAttrs}>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs>
								<Table.Cell {...attrs}>
									<Render of={cell.render()} />
								</Table.Cell>
							</Subscribe>
						{/each}
					</Table.Row>
				</Subscribe>
			{/each}
		</Table.Body>
	</Table.Root>
</div>

