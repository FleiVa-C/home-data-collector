<script>
	import { Input } from "$lib/components/ui/input/index";
	import { Button } from "$lib/components/ui/button/index";
	import { fade } from 'svelte/transition';
</script>

<div
	class="container relative hidden h-screen flex-col items-center justify-center md:grid lg:max-w-none lg:grid-cols-2 lg:px-0"
>
	<div class="relative hidden h-full flex-col p-10 text-white lg:flex dark:border-r">
		<div
			class="absolute inset-0 bg-cover"
			style="
			background-image:
				url(new_york.jpeg);"
		/>
	</div>
	<div class="lg:p-8">
		<div class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
			<div class="flex flex-col space-y-2 text-center">
				<h1 class="text-2xl font-semibold tracking-tight">Create Account</h1>
				<div class="flex flex-row justify-center">
					<p class="text-muted-foreground text-sm">
						Already have an Account ?
					</p>
					<a href="/login"class="ml-1 text-muted-foreground text-sm hover:text-foreground underline"> Log in.</a>
				</div>
			</div>
			<div class="flex flex-col gap-4">
				<Input type="text" placeholder="firstname"/>
				<Input type="text" placeholder="lastname"/>
				<Input type="email" placeholder="email"/>
				<Input type="password" placeholder="password"/>
				<Button>Create Account</Button>
			</div>
		</div>
	</div>
</div>

<style>
</style>
