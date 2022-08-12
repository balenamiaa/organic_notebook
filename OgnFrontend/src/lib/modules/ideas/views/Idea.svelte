<script>
	import { DocumentsContextKey } from '$lib/modules/documents/stores.js';

	import { getContext } from 'svelte';
	import {
		createIdea,
		deleteIdea,
		deleteIdeaRef,
		deleteIdeaRefsForIdea,
		getIdeaRefsForIdea,
	} from '../api.js';
	import { IdeasContextKey } from '../stores.js';

	const { ideasContext } = getContext(IdeasContextKey);
	const { documentsContext } = getContext(DocumentsContextKey);
	let currentIdea;
	let ideaRefsForCurrentIdea;
	ideasContext.refresh();

	$: if (currentIdea) {
		populateCurrentIdeaRefsForIdea(currentIdea.id);
	}

	$: if ($ideasContext.actions.length > 0) {
		const index = $ideasContext.actions.length - 1;
		const action = $ideasContext.actions[index];
		switch (action.type) {
			case 'refresh-idea-ref':
				{
					populateCurrentIdeaRefsForIdea(action.payload.ideaId);
					ideasContext.removeAction(index);
				}
				break;
		}
	}

	async function onSubmit(event) {
		try {
			const ideaLabel = event.target.elements['idea_label'].value;
			const response = await createIdea({ label: ideaLabel });

			if (response.status === 200) {
				ideasContext.refresh();
			}
		} catch (err) {}
	}
	async function onRemoveIdea(ideaId) {
		try {
			await deleteIdeaRefsForIdea(ideaId);
			const response = await deleteIdea(ideaId);
			if (response.status === 200) {
				ideasContext.refresh();
			}
		} catch (err) {}
	}
	async function onRemoveIdeaRef(ideaRefId) {
		try {
			const response = await deleteIdeaRef(ideaRefId.id);
			if (response.status === 200) {
				populateCurrentIdeaRefsForIdea(currentIdea.id);
			}
		} catch (err) {}
	}
	async function populateCurrentIdeaRefsForIdea(ideaId) {
		try {
			const response = await getIdeaRefsForIdea(ideaId);
			if (response.status === 200) {
				ideaRefsForCurrentIdea = await response.json();
			}
		} catch (err) {}
	}
	function viewDocument(doc) {
		documentsContext.pushAction({
			type: 'open-document',
			payload: {
				pageNumber: doc.page_number,
				documentId: doc.document_id,
			},
		});
	}
</script>

<div>
	<h2>Ideas</h2>
	<form on:submit|preventDefault={onSubmit}>
		Create Idea
		<input type="text" name="idea_label" placeholder="Idea label" />
		<div>
			<button>Submit</button>
		</div>
	</form>
	<h3>List</h3>
	{#if $ideasContext.ideas.items}
		<ol>
			{#each $ideasContext.ideas.items as idea}
				<li>
					<span>
						{idea.label}
					</span>
					<button on:click={() => (currentIdea = idea)}>View ref</button>
					<button on:click={() => onRemoveIdea(idea.id)} class="warning-color">Remove</button>
				</li>
			{/each}
		</ol>
		{#if currentIdea && ideaRefsForCurrentIdea}
			<h3>{currentIdea.label}</h3>
			<ol>
				{#each ideaRefsForCurrentIdea.items as ideaRef}
					<li>
						<span>
							{ideaRef.idea_ref_text} | doc: {documentsContext.getDocumentById(
								$documentsContext.documents,
								ideaRef.doc_page.document_id,
							).title} page: {ideaRef.doc_page.page_number}
						</span>
						<button on:click={() => viewDocument(ideaRef.doc_page)}>View</button>
						<button on:click={() => onRemoveIdeaRef(ideaRef)} class="warning-color">Remove</button>
					</li>
				{/each}
			</ol>
		{/if}
	{/if}
</div>
