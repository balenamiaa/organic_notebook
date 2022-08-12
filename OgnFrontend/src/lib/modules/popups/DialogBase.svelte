<script>
	import Portal from './Portal.svelte';
	import { createEventDispatcher, onDestroy, onMount } from 'svelte';
	import { fade } from 'svelte/transition';
	import { outClickEvent } from '$lib/utils/events/outClickEvent';
	import { browser } from '$app/env';

	const dispatch = createEventDispatcher();
	onMount(() => {
		if (browser) document.body.style.overflow = 'hidden';
	});
	onDestroy(() => {
		if (browser) document.body.style.overflow = '';
	});
</script>

<Portal>
	<div
		id="root"
		style="left: {window.scrollX}px; top: {window.scrollY}px;"
		transition:fade={{ delay: 0, duration: 100 }}
	>
		<div
			id="dialog"
			use:outClickEvent
			on:outClick={() => {
				dispatch('close');
			}}
		>
			<slot />
		</div>
	</div>
</Portal>

<style>
	#root {
		position: absolute;
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: rgba(0, 0, 0, 0.322);
	}
	#dialog {
		background-color: var(--background-paper);
		box-shadow: var(--elevation-5);
		border-radius: var(--border-radius);
		padding: var(--spacing);
		margin: var(--spacing);
		display: flex;
		flex-direction: column;
		overflow: auto;
		max-height: calc(100% - 2 * var(--spacing));
	}
</style>
