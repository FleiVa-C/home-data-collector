<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, type PopupSettings } from '@skeletonlabs/skeleton';

	// Highlight JS
	import hljs from 'highlight.js/lib/core';
	import 'highlight.js/styles/github-dark.css';
	import { storeHighlightJs, Avatar } from '@skeletonlabs/skeleton';
	import xml from 'highlight.js/lib/languages/xml'; // for HTML
	import css from 'highlight.js/lib/languages/css';
	import javascript from 'highlight.js/lib/languages/javascript';
	import typescript from 'highlight.js/lib/languages/typescript';

	hljs.registerLanguage('xml', xml); // for HTML
	hljs.registerLanguage('css', css);
	hljs.registerLanguage('javascript', javascript);
	hljs.registerLanguage('typescript', typescript);
	storeHighlightJs.set(hljs);

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup, popup } from '@skeletonlabs/skeleton';
    import type { PageData } from './$types';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });


    const popupCLick: PopupSettings = {
        event: 'click',
        target: 'popupClick',
        placement: 'bottom'
    };

    export let data: PageData;
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar padding="p-1">
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase">
                    <a href="/">HDC</a>
                </strong>
			</svelte:fragment>
			<svelte:fragment slot="default">
				<a
					class="btn btn-md hover:variant-soft-surface"
					href="/dashboard"
				>
					Dashboard
				</a>
				<a
					class="btn btn-md hover:variant-soft-surface"
					href="/interface"
				>
					Interface
				</a>
				<a
					class="btn btn-md hover:variant-soft-surface"
					href="/smte"
				>
					Something Else
				</a>
            </svelte:fragment>
            <svelte:fragment slot="trail">
                    <form method="POST">
                        {#if !data.user}
                            <a class="btn btn-md hover:variant-soft-surface" href="/login">Login</a>
                            <a class="btn btn-md hover:variant-soft-surface" href="/signup">signup</a>
                        {:else}
                            <button formaction="/logout" class="btn btn-md hover:variant-soft-surface" type="submit">Logout</button>
                        {/if}
                    </form>
                <button use:popup={popupCLick}>
                    <Avatar src="dummy_image.png" initials="FR" width="w-10" class="mr-0 p-0"/>
                </button>
            </svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>


<div class="card p-4 -my-1 variant-filled-background" data-popup="popupClick">
<div class="grid grid-cols-1 gap-2">
	<a href="/profile">Profile</a>
	<a href="/">Settings</a>
	<a href="/">Admin</a>
</div>
	<div class="arrow variant-filled-primary"/>
</div>
