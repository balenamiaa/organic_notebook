<script>
	import { getContext } from 'svelte';
	import {
		deleteDocument,
		deleteExtractedTextsForDocument,
		deleteIdeaRefsForDocument,
		extractDocumentText,
		uploadDocument,
	} from '../api.js';
	import { DocumentsContextKey } from '../stores';
	import DocumentsSearch from './DocumentsSearch.svelte';
	import DocumentView from './DocumentView.svelte';

	const { documentsContext } = getContext(DocumentsContextKey);
	let currentDoc;
	let currentPage = -1;
	documentsContext.refresh();
	documentsContext.refreshExtractedTexts();

	$: if ($documentsContext.actions.length > 0) {
		const index = $documentsContext.actions.length - 1;
		const action = $documentsContext.actions[index];
		switch (action.type) {
			case 'open-document':
				{
					currentDoc = documentsContext.getDocumentById(
						$documentsContext.documents,
						action.payload.documentId,
					);
					currentPage = action.payload.pageNumber;
				}
				break;
		}
		documentsContext.removeAction(index);
	}

	async function onSubmit(event) {
		try {
			const files = event.target.elements['files'].files;
			const response = await uploadDocument(files);

			if (response.status === 200) {
				await documentsContext.refresh();
			}

			const json = await response.json();

			if (json.length > 0) {
				await Promise.all(json.map((doc) => extractDocumentText(doc.id)));
			}
		} catch (err) {}
	}
	async function onRemove(documentId) {
		try {
			await deleteExtractedTextsForDocument(documentId);
			await deleteIdeaRefsForDocument(documentId);

			const response = await deleteDocument(documentId);
			if (response.status === 200) {
				await documentsContext.refresh();
			}
		} catch (err) {}
	}
	async function onExtractText(documentId) {
		try {
			extractDocumentText(documentId);
			documentsContext.refreshExtractedTexts();
		} catch (err) {}
	}
</script>

<div>
	<h2>Documents</h2>
	<DocumentsSearch />
	<form on:submit|preventDefault={onSubmit}>
		Select a document
		<input type="file" name="files" multiple />
		<div>
			<button>Submit</button>
		</div>
	</form>
	<h3>List</h3>
	{#if $documentsContext.documents.items}
		<ol>
			{#each $documentsContext.documents.items as doc}
				<li>
					<span class:selected-document={doc === currentDoc}>
						{doc.title}
					</span>
					<button on:click={() => (currentDoc = doc)}>View</button>
					<button on:click={() => onExtractText(doc.id)}>Extract text</button>
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
