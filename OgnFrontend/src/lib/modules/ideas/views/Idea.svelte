<script>
	import { documentsKey } from '$lib/modules/documents/stores.js'

	import { groupBySingle } from '$lib/utils/array-utils.js'
	import { getContext } from 'svelte'
	import { createIdea, deleteIdea, deleteIdeaRef, getIdeaEntry, getIdeaRefs } from '../api.js'
	import { ideasKey } from '../stores.js'

	const { ideas } = getContext(ideasKey)
	const { documents } = getContext(documentsKey)
	let currentIdea
	let currentIdeaRefs
	ideas.refresh()

	$: if (currentIdea) {
		fetchIdeaRefs(currentIdea.id)
	}

	$: if ($ideas.actions.length > 0) {
		const index = $ideas.actions.length - 1
		const action = $ideas.actions[index]
		switch (action.type) {
			case 'refresh-idea-ref':
				{
					fetchIdeaRefs(action.payload.ideaId)
					ideas.removeAction(index)
				}
				break
		}
	}

	async function onSubmit(event) {
		try {
			const ideaLabel = event.target.elements['idea_label'].value
			const response = await createIdea({ label: ideaLabel })

			if (response.status === 200) {
				ideas.refresh()
			}
		} catch (err) {}
	}
	async function onRemoveIdea(ideaId) {
		try {
			const response = await deleteIdea(ideaId)
			if (response.status === 200) {
				ideas.refresh()
			}
		} catch (err) {}
	}
	async function onRemoveIdeaRef(ideaRefId) {
		try {
			const response = await deleteIdeaRef(ideaRefId.id)
			if (response.status === 200) {
				fetchIdeaRefs(currentIdea.id)
			}
		} catch (err) {}
	}
	async function fetchIdeaRefs(ideaId) {
		try {
			const response = await getIdeaRefs(ideaId)
			if (response.status === 200) {
				currentIdeaRefs = await response.json()
			}
		} catch (err) {}
	}
	function viewDocument(doc) {
		documents.pushAction({
			type: 'open-document',
			payload: {
				pageNumber: doc.page_number,
				documentId: doc.document_id,
			},
		})
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
	{#if $ideas.ideas}
		<ol>
			{#each $ideas.ideas as idea}
				<li>
					<span>
						{idea.label}
					</span>
					<button on:click={() => (currentIdea = idea)}>View ref</button>
					<button on:click={() => onRemoveIdea(idea.id)} class="warning-color">Remove</button>
				</li>
			{/each}
		</ol>
		{#if currentIdea && currentIdeaRefs}
			<h3>{currentIdea.label}</h3>
			<ol>
				{#each currentIdeaRefs.idea_refs as ideaRef}
					<li>
						<span>
							{ideaRef.idea_ref_text} | doc: {documents.getDocumentById(
								$documents.documents,
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
