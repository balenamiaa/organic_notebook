<script>
	import { outClickEvent } from '$lib/utils/events/outClickEvent'
</script>

<div id="root" use:outClickEvent on:outClick>
	<slot />
</div>

<style>
	#root {
		background-color: white;
		padding: 6px;
		border-radius: var(--border-radius);
		width: fit-content;
		z-index: 10000;
		box-shadow: var(--elevation-3);
	}
</style>
