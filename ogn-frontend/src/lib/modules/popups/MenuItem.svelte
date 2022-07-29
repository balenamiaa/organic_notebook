<div id="root" on:mousedown|stopPropagation on:click>
	<slot />
</div>

<style>
	#root {
		pointer-events: painted;
		width: 100%;
		pointer-events: all;
		cursor: pointer;
		padding: 8px;
		border: 3px;
	}
	#root:hover {
		background-color: var(--action-hover);
	}
	#root:active {
		background-color: var(--action-selected);
	}
</style>
