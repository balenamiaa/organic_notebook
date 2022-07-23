<script>
	import { createIdea, createIdeaReference } from '$lib/modules/ideas/api'

	import { ideasKey } from '$lib/modules/ideas/stores'

	import PdfDocument from '$lib/modules/pdf/views/PdfDocument.svelte'
	import PdfPage from '$lib/modules/pdf/views/PdfPage.svelte'
	import Menu from '$lib/modules/popups/Menu.svelte'
	import MenuItem from '$lib/modules/popups/MenuItem.svelte'
	import Popups from '$lib/modules/popups/Popups.svelte'
	import { baseUrl } from '$lib/utils/api'
	import { getContext } from 'svelte'

	export let doc

	const { ideas } = getContext(ideasKey)

	let numPages = 0
	let showSelectionMenu = false
	let menuPos = null
	let selectionText = ''
	let ideaAlreadyExis

	function onSelectionEnd(event) {
		menuPos = event.detail.posInScreen
		selectionText = event.detail.selectionText
		showSelectionMenu = true
		ideaAlreadyExis = $ideas.ideas.findIndex((idea) => idea.label === selectionText) !== -1
	}
	function onSelectionChange(event) {
		showSelectionMenu = false
	}
	async function onIdeaClick(idea) {
		const response = await createIdeaReference({
			idea_ref: idea.id,
			doc_page: {
				document_id: doc.id,
				page_number: 1,
			},
		})
		if (response.status === 200) {
			ideas.refresh()
		}
		showSelectionMenu = false
	}
	async function onCreateNewIdeaClick() {
		let response = await createIdea({ label: selectionText })
		const idea = await response.json()

		response = await createIdeaReference({
			idea_ref: idea.id,
			doc_page: {
				document_id: doc.id,
				page_number: 1,
			},
		})
		if (response.status === 200) {
			ideas.refresh()
		}
		showSelectionMenu = false
	}
</script>

{#if doc.filetype === 'pdf'}
	<PdfDocument
		srcUrl={`${baseUrl}/static/${doc.id}.${doc.filetype}`}
		on:loadSuccess={(event) => {
			numPages = event.detail.numPages
		}}
		on:selectionEnd={onSelectionEnd}
		on:selectionChange={onSelectionChange}
	>
		<div>
			{#each Array(numPages) as _, i}
				<PdfPage pageNumber={i + 1} />
			{/each}
		</div>
	</PdfDocument>
{:else}
	<div class="warning-color">Sorry this format currently is not supported</div>
{/if}

{#if showSelectionMenu}
	<Popups show={true} top={menuPos?.y} left={menuPos?.x}>
		<Menu on:outClick={() => (showSelectionMenu = false)}>
			{#if !ideaAlreadyExis}
				<MenuItem on:click={onCreateNewIdeaClick}>Create</MenuItem>
			{/if}
			{#each $ideas.ideas as idea}
				<MenuItem on:click={() => onIdeaClick(idea)}>{idea.label}</MenuItem>
			{/each}
		</Menu>
	</Popups>
{/if}
