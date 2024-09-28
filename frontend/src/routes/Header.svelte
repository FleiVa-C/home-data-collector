<script>
	import { Sun, Moon } from 'radix-icons-svelte';
	import { toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/index';
	import { Separator } from "$lib/components/ui/separator/index";
	import * as Avatar from "$lib/components/ui/avatar/index";
</script>

<header class="flex flex-row flex-nowrap p-1 gap-6 ml-4">
	<div class="self-center">
		<a href="/">
			<h1 class="content-center">HDC</h1>
		</a>
	</div>
	<nav class="text-sm flex flex-row grow gap-4 self-center">
		<a href="/interface" class="text-foreground/60 hover:text-foreground/80">
			<h2>Interface</h2>
		</a>
		<a href="/dashboard" class="text-foreground/60 hover:text-foreground/80">
		<h2>Dashboard</h2>
		</a>
	</nav>

	<div class="flex flex-row w-auto gap-1">
	<Button on:click={toggleMode} variant="ghost" size="icon">
	  <Sun class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"/>
	  <Moon class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"/>
	  <span class="sr-only">Toggle theme</span>	
	</Button>
	<Avatar.Root class="h-7 w-7 self-center">
		<Avatar.Image src="dummy_image.png" alt="@dummy"/>
		<Avatar.Fallback>FR</Avatar.Fallback>
	</Avatar.Root>
	</div>


</header>
<Separator/>
<style>
	/**{*/
	/*	outline: red;*/
	/*	outline-width: 1px;*/
	/*	outline-style: solid;*/
	/*	}*/
</style>

