<script>
import { baseUrl } from '$lib/utils/api.js';

	import { getContext } from 'svelte'
	import { getDocuments, uploadDocument } from '../api.js'
	import { documentsKey } from '../stores'

	const { documents } = getContext(documentsKey)

	refreshDocuments()

	async function onSubmit(event) {
		try {
			const files = event.target.elements['files'].files
			const response = await uploadDocument(files)

			if (response.status === 200) {
				refreshDocuments()
			}
		} catch (err) {}
	}
	async function refreshDocuments() {
		try {
			$documents = await (await getDocuments()).json()
		} catch (err) {}
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
	{#if $documents.documents}
		<ol>
			{#each $documents.documents as doc}
				<li>
					{doc.title}
					<a target="_blank" href={`${baseUrl}/static/${doc.id}.${doc.filetype}`}>View</a>
				</li>
			{/each}
		</ol>
	{/if}
</div>
