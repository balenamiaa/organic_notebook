<script>
	import PdfDocument from '$lib/modules/pdf/views/PdfDocument.svelte'
	import PdfPage from '$lib/modules/pdf/views/PdfPage.svelte'
	import { baseUrl } from '$lib/utils/api'

	export let document
	let numPages = 0
</script>

{#if document.filetype === 'pdf'}
	<PdfDocument
		srcUrl={`${baseUrl}/static/${document.id}.${document.filetype}`}
		on:loadSuccess={(event) => {
			numPages = event.detail.numPages
		}}
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
