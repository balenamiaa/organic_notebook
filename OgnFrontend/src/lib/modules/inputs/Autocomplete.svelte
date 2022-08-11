<script>
	import { outClickEvent } from '$lib/utils/events/outClickEvent'
	import { createEventDispatcher } from 'svelte'
	import Divider from '$lib/modules/others/Divider.svelte'

	export let name = ''
	export let placeholder = 'Enter text here...'
	export let options = ['All about AJAM', 'All about BAAA', 'All about RRAA']
	export let maxWidth = '260px'
	export let showNoItem = true
	export let getText = (value, inputValue) => value
	export let autocomplete = true

	const dispatch = createEventDispatcher()
	let showAutocompleteList = false
	let input
	let inputValue = ''
</script>

<div
	id="root"
	style="max-width: {maxWidth};"
	use:outClickEvent
	on:outClick={() => (showAutocompleteList = false)}
>
	<div class="input-root" on:click={() => input.focus()}>
		<input
			{placeholder}
			{name}
			autocomplete="off"
			class="input"
			bind:this={input}
			on:focus={() => (showAutocompleteList = true)}
			on:keydown={(event) => {
				if (!autocomplete) true
				if (event.key === 'Enter') {
					const option = options.find(
						(op) => getText(op, inputValue).toLowerCase() === input.value.toLowerCase(),
					)
					if (option) dispatch('optionSelect', { option })
				}
			}}
			on:input
			bind:value={inputValue}
		/>
	</div>
	{#if autocomplete}
		{#if showAutocompleteList && options?.length > 0}
			<div class="autocomplete-root">
				{#each options as option}
					<div
						on:click={() => {
							dispatch('optionSelect', { option })
							showAutocompleteList = false
						}}
						class="autocomplete-item pointer hover-bg selected-bg"
					>
						<slot name="item" {option} {inputValue}>
							{getText(option, inputValue)}
						</slot>
					</div>
					<Divider />
				{/each}
			</div>
		{:else if showNoItem && showAutocompleteList}
			<div class="autocomplete-root">
				<div
					on:click={() => {
						showAutocompleteList = false
					}}
					class="autocomplete-item pointer hover-bg selected-bg"
				>
					<slot name="noItem" {inputValue}>No items</slot>
				</div>
				<Divider />
			</div>
		{/if}
	{/if}
</div>

<style>
	#root {
		position: relative;
		margin-bottom: var(--spacing);
	}
	.input-root {
		background-color: var(--background-paper);
		display: flex;
		padding: 6px 6px;
		gap: 6px;
		border-radius: var(--border-radius);
		border: 1.4px solid var(--grey-400);
		align-items: center;
		flex-wrap: wrap;
		height: fit-content;
	}
	input {
		border: none;
		outline: none;
		font-size: 1.05rem;
		color: currentColor;
		font-family: var(--main-font-family);
		width: 80px;
		flex: 1 1;
		padding: 0;
		margin: 0;
	}
	.autocomplete-root {
		position: absolute;
		border-radius: var(--border-radius);
		box-shadow: var(--elevation-3);
		z-index: var(--modal-zindex);
		max-height: 300px;
		background-color: var(--background-paper);
		width: 100%;
		overflow-y: auto;
	}
	.autocomplete-item {
		padding: var(--spacing);
	}
</style>
