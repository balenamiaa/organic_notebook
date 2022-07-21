<script>
	import { getContext } from 'svelte'
	import { uploadDocument } from '../api.js'
	import { documentsKey } from '../stores'

	const { documents } = getContext(documentsKey)

	function onSubmit(event) {
		uploadDocument(event.target.elements['files'].files)
	}
</script>

<div>
	<h2>Documents</h2>
	<form on:submit|preventDefault={onSubmit}>
		Select a document
		<input type="file" name="files" multiple />
		<div>
			<button>Submit</button>
		</div>
	</form>
	<h3>List</h3>
	{#if $documents.list}
		<ol>
			{#each $documents.list as doc}
				<li>{doc}</li>
			{/each}
		</ol>
	{/if}
</div>
