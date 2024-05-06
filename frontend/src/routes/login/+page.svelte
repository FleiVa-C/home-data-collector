<script lang="ts">
    import { enhance } from "$app/forms";
    import { page } from "$app/stores";
    import { fade } from 'svelte/transition';

    let message: string

    $: message = $page.url.searchParams.get("message") ?? ""
</script>
<head>
    <title>Sign In</title>
</head>

<div class="sm mx-auto w-96 mt-24">
    {#if message}
        <div transition:fade class="alert-message variant-filled-warning mb-4">
        <p class="pl-1">{message}</p>
    </div>
    {/if}
    <div class="card p-4">
        <strong>Login</strong>
            <form method="post" use:enhance>
                <div class="form-element">
                    <label for="username">Username</label>
                    <input type="text" name="username" id="username" class="input variant-form-material !rounded-none"/><br />
                </div>
                <div class="form-element">
                    <label for="password">Password</label>
                    <input type="password" name="password" id="password" class="input variant-form-material !rounded-none" placeholder="password"/><br />
                </div>
                <button class="variant-ghost-success mt-4 py-0.5 px-4 w-full">login</button>
            </form>
        <div class="flex space-x-8 mt-4">
            <p> No Account ?</p>
            <a class="anchor" href="/signup">Signup</a>
        </div>
    </div>
</div>

<style>
    .form-element{
        margin-top: 1rem;
    }
</style>
