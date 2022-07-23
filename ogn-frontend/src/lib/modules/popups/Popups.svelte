<script>
	import Portal from './Portal.svelte'

	export let show = false
	export let posUnit = 'px'
	export let top = 0
	export let left = 0
</script>

{#if show}
	<Portal>
		<div id="root" style="top: {top}{posUnit}; left: {left}{posUnit};">
			<slot />
		</div>
	</Portal>
{/if}

<style>
	#root {
		position: absolute;
		pointer-events: none;
		width: 100%;
		height: 100%;
		width: fit-content;
		height: fit-content;
	}
</style>
