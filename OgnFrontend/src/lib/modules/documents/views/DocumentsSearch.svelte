<script>
	import Autocomplete from '$lib/modules/inputs/Autocomplete.svelte'
	import { getContext } from 'svelte'
	import { documentsKey } from '../stores'
	import DocumentView from './DocumentView.svelte'

	let options = []

	const { documents } = getContext(documentsKey)
	const searchOptions = {
		maxCharsShowPreMatch: 10,
		maxCharsShowAfterMatch: 10,
		lineBreak: true,
		/**
		 * @type {'autocomplete' | 'preview-page'}
		 */
		resultStyle: 'autocomplete',
	}

	$: if (searchOptions.resultStyle === 'preview-page') {
	}

	function onInput(term) {
		if (!term) {
			options = []
			return
		}
		options = $documents.extractedTexts.filter((extractedText) => {
			return extractedText.content.includes(term)
		})
	}
	function getText(extractedText, term) {
		const regex = new RegExp(term, 'g')
		let content
		if (searchOptions.lineBreak) {
			content = extractedText.content
		} else {
			content = extractedText.content.replaceAll('\n', ' ')
		}
		const matches = [...content.matchAll(regex)]
		let out = ''
		let prevIndex = 0
		for (let i = 0; i < matches.length; i++) {
			const match = matches[i]
			if (match.index - prevIndex < searchOptions.maxCharsShowPreMatch) {
				out += content.slice(match.index, match.index + term.length)
			} else {
				out +=
					'...' +
					content.slice(
						match.index - searchOptions.maxCharsShowPreMatch,
						match.index + term.length + searchOptions.maxCharsShowAfterMatch,
					)
			}
			prevIndex = match.index
		}
		return out
	}
	function viewDocument(extractedText) {
		documents.pushAction({
			type: 'open-document',
			payload: {
				pageNumber: extractedText.doc_page.page_number,
				documentId: extractedText.doc_page.document_id,
			},
		})
	}
</script>

<h3>Search options</h3>
<form>
	<select bind:value={searchOptions.resultStyle} placeholder="resultStyle">
		{#each ['autocomplete', 'preview-page'] as op}
			<option value={op}>{op}</option>
		{/each}
	</select>
	{#if searchOptions.resultStyle === 'autocomplete'}
		<input
			type="number"
			placeholder="maxCharsShowPreMatch"
			bind:value={searchOptions.maxCharsShowPreMatch}
		/>
		<input
			type="number"
			placeholder="maxCharsShowAfterMatch"
			bind:value={searchOptions.maxCharsShowAfterMatch}
		/>
		<label>
			lineBreak
			<input id="lineBreak" type="checkbox" bind:checked={searchOptions.lineBreak} />
		</label>
	{/if}
</form>
<br />
<Autocomplete
	placeholder="Search in documents"
	autocomplete={searchOptions.resultStyle === 'autocomplete'}
	{options}
	{getText}
	maxWidth="700px"
	on:optionSelect={(event) => viewDocument(event.detail.option)}
	on:input={(event) => onInput(event.target.value)}
>
	<svelte:fragment slot="item" let:option let:inputValue>
		Document: <strong
			>{documents.getDocumentById($documents.documents, option.doc_page.document_id).title}</strong
		>
		Page: <strong>{option.doc_page.page_number}</strong>
		<br />

		{@html getText(option, inputValue)
			.replace(new RegExp(inputValue, 'gi'), (match) => `<strong>${match}</strong>`)
			.replace(new RegExp('\n', 'gi'), '<br>')}
	</svelte:fragment>
	<svelte:fragment slot="noItem" let:inputValue>
		{#if inputValue.length === 0}
			Type something...
		{:else}
			Nothing found
		{/if}
	</svelte:fragment>
</Autocomplete>

{#if searchOptions.resultStyle === 'preview-page'}
	{#each options as extractedText}
		<DocumentView
			doc={documents.getDocumentById($documents.documents, extractedText.doc_page.document_id)}
			currentPage={extractedText.doc_page.page_number}
			onlyShowCurrentPage={true}
		/>
	{/each}
{/if}
