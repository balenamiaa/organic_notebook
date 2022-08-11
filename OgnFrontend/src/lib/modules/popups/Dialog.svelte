<script>
	import { createEventDispatcher } from 'svelte'
	import CloseSvg from '$lib/utils/icons/svg/CloseSvg.svelte'
	import DialogBase from './DialogBase.svelte'

	const dispatch = createEventDispatcher()
</script>

<DialogBase on:close>
	<div class="flex-end">
		<div id="title">
			<slot name="title" />
		</div>
		<button
			class="button-base"
			on:click={() => {
				dispatch('close')
			}}
		>
			<CloseSvg />
		</button>
	</div>
	<div class="flex-column">
		<slot />
	</div>
</DialogBase>

<style>
	#title {
		font-size: 1.7rem;
		font-weight: bold;
		flex: 1 1;
		text-align: center;
      padding-right: calc(6 * var(--spacing));
	}
</style>
