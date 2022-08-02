<script>
	import { createIdea, createIdeaReference } from '$lib/modules/ideas/api'

	import { ideasKey } from '$lib/modules/ideas/stores'

	import PdfDocument from '$lib/modules/pdf/views/PdfDocument.svelte'
	import PdfPage from '$lib/modules/pdf/views/PdfPage.svelte'
	import Dialog from '$lib/modules/popups/Dialog.svelte'
	import Menu from '$lib/modules/popups/Menu.svelte'
	import MenuItem from '$lib/modules/popups/MenuItem.svelte'
	import Popups from '$lib/modules/popups/Popups.svelte'
	import { baseUrl } from '$lib/utils/api'
	import { getContext, tick } from 'svelte'

	export let doc
	export let currentPage = -1
	export let searchTerm = ''
	export let onlyShowCurrentPage = false

	const { ideas } = getContext(ideasKey)

	let numPages = 0
	let showSelectionMenu = false
	let menuPos = null
	let selectionText = ''
	let selectionPage = 0
	let ideaAlreadyExist
	let documentFile
	let moreOption = false

	$: if (documentFile && currentPage != -1) {
		tick().then(() => {
			moveToPage(doc.id, currentPage)
		})
	}

	function onSelectionEnd(event) {
		menuPos = event.detail.posInScreen
		selectionText = event.detail.selectionText
		selectionPage = Number(
			event.detail.focusNode.parentElement.closest('[data-page-number]').dataset.pageNumber,
		)
		showSelectionMenu = true
		ideaAlreadyExist = $ideas.ideas.findIndex((idea) => idea.label === selectionText) !== -1
	}
	function onSelectionChange(event) {
		showSelectionMenu = false
	}
	async function onIdeaRefSubmit(event) {
		const ideaId = event.target.elements['idea-id'].value
		const response = await createIdeaReference({
			idea_ref: ideaId,
			idea_ref_text: selectionText,
			doc_page: {
				document_id: doc.id,
				page_number: selectionPage,
			},
		})
		if (response.status === 200) {
			ideas.pushAction({ type: 'refresh-idea-ref', payload: { ideaId } })
		}
		showSelectionMenu = false
	}
	async function onCreateNewIdeaClick() {
		let response = await createIdea({ label: selectionText })
		const idea = await response.json()

		response = await createIdeaReference({
			idea_ref: idea.id,
			idea_ref_text: selectionText,
			doc_page: {
				document_id: doc.id,
				page_number: selectionPage,
			},
		})
		if (response.status === 200) {
			ideas.refresh()
		}
		showSelectionMenu = false
	}
	async function moveToPage(documentId, page) {
		if (doc.filetype === 'pdf') {
			document
				.querySelector(`[data-document-id="${documentId}"] [data-page-number="${page}"]`)
				.scrollIntoView()
		}
	}
</script>

{#if doc.filetype === 'pdf'}
	<PdfDocument
		srcUrl={`${baseUrl}/static/${doc.id}.${doc.filetype}`}
		on:loadSuccess={(event) => {
			numPages = event.detail.numPages
			documentFile = event.detail
		}}
		documentId={doc.id}
		on:selectionEnd={onSelectionEnd}
		on:selectionChange={onSelectionChange}
	>
		<div>
			{#if onlyShowCurrentPage}
				<PdfPage pageNumber={currentPage} {searchTerm}/>
			{:else}
				{#each Array(numPages) as _, i}
					<PdfPage pageNumber={i + 1} {searchTerm}/>
				{/each}
			{/if}
		</div>
	</PdfDocument>
{:else}
	<div class="warning-color">Sorry this format currently is not supported</div>
{/if}

{#if showSelectionMenu}
	<Popups show={!moreOption} top={menuPos?.y} left={menuPos?.x}>
		<Menu on:outClick={() => (showSelectionMenu = false)}>
			{#if !ideaAlreadyExist}
				<MenuItem on:click={onCreateNewIdeaClick}>Create idea and ref</MenuItem>
			{/if}
			<MenuItem on:click={() => (moreOption = true)}>More options</MenuItem>
		</Menu>
	</Popups>
	{#if moreOption}
		<Dialog
			on:close={() => {
				showSelectionMenu = false
				moreOption = false
			}}
		>
			<svelte:fragment slot="title"
				>Reference <span style="font-style: italic;">{selectionText}</span></svelte:fragment
			>
			<form on:submit|preventDefault={onIdeaRefSubmit}>
				<label for="idea-id">Idea</label>
				<select id="idea-id" name="idea-id">
					{#each $ideas.ideas as idea}
						<option value={idea.id}>{idea.label}</option>
					{/each}
				</select>
				<div>
					<button type="submit">Submit</button>
				</div>
			</form>
		</Dialog>
	{/if}
{/if}

<style>
</style>
