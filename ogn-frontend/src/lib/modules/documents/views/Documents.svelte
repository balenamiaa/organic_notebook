<script>
	import { getContext } from 'svelte'
	import { deleteDocument, uploadDocument } from '../api.js'
	import { documentsKey } from '../stores'
	import DocumentView from './DocumentView.svelte'

	const { documents } = getContext(documentsKey)
	let currentDoc
	let currentPage = -1
	documents.refresh()

	$: if ($documents.actions.length > 0) {
		const index = $documents.actions.length - 1
		const action = $documents.actions[index]
		switch (action.type) {
			case 'open-document':
				{
					currentDoc = documents.getDocumentById($documents.documents, action.payload.documentId)
					currentPage = action.payload.pageNumber
				}
				break
		}
		documents.removeAction(index)
	}

	async function onSubmit(event) {
		try {
			const files = event.target.elements['files'].files
			const response = await uploadDocument(files)

			if (response.status === 200) {
				await documents.refresh()
			}
		} catch (err) {}
	}
	async function onRemove(documentId) {
		try {
			const response = await deleteDocument(documentId)
			if (response.status === 200) {
				await documents.refresh()
			}
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
					<span class:selected-document={doc === currentDoc}>
						{doc.title}
					</span>
					<button on:click={() => (currentDoc = doc)}>View</button>
					<button on:click={() => onRemove(doc.id)} class="warning-color">Remove</button>
				</li>
			{/each}
		</ol>
	{/if}
	{#if currentDoc}
		<DocumentView doc={currentDoc} {currentPage} />
	{/if}
</div>

<style>
	.selected-document {
		background-color: yellow;
	}
</style>
