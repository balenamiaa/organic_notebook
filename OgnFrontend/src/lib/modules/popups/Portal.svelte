<!-- https://stackoverflow.com/questions/62733094/implement-a-portal-in-svelte -->
<script>
	import { browser } from '$app/env';

	import { onMount, onDestroy } from 'svelte';
	let ref;
	let portal;

	onMount(() => {
		portal = document.createElement('div');
		portal.className = 'portal';
		document.body.appendChild(portal);
		portal.appendChild(ref);
	});

	onDestroy(() => {
		if (browser) document.body.removeChild(portal);
	});
</script>

<!-- See this issue : https://github.com/sveltejs/svelte/issues/3088 Portal.svelte -->

<div class="portal-clone">
	<div bind:this={ref}>
		<slot />
	</div>
</div>

<style>
	.portal-clone {
		display: none;
	}
</style>
