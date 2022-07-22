<svelte:options immutable={true} />

<script>
	import { getContext, tick } from 'svelte'
	import { PdfKey } from '../stores'
	import * as pdfjs from 'pdfjs-dist'

	export let pageNumber

	const pdf = getContext(PdfKey)
	let page
	let viewport
	let canvas
	let textContainer
	let pageRenderTask
	let textRenderTask

	$: if ($pdf) {
		refreshPage($pdf)
	}
	$: if (page && viewport) {
		tick().then(() => {
			renderPage(page, viewport)
			renderText(page, viewport)
		})
	}

	async function refreshPage(pdf) {
		const scale = 1.5
		try {
			page = await pdf.getPage(pageNumber)
			viewport = page.getViewport({ scale: scale })
		} catch (err) {}
	}
	async function renderPage(page, viewport) {
		if (pageRenderTask?._internalRenderTask.running) pageRenderTask.cancel()
		const context = canvas.getContext('2d')
		canvas.height = viewport.height
		canvas.width = viewport.width

		const renderContext = {
			canvasContext: context,
			viewport: viewport,
		}
		pageRenderTask = page.render(renderContext)
	}
	async function renderText(page, viewport) {
		textRenderTask?.cancel()
		while (textContainer.firstChild) textContainer.firstChild.remove()
		textRenderTask = pdfjs.renderTextLayer({
			container: textContainer,
			textContentStream: page.streamTextContent(),
			viewport,
		})
	}
</script>

<div id="root">
	<canvas bind:this={canvas} />

	<div bind:this={textContainer} id="text-container" />
</div>

<style>
	#root {
		position: relative;
	}

	#text-container {
		position: absolute;
		overflow: clip;
		inset: 0;
		opacity: 0.2;
		line-height: 1;
	}

	div > :global(span) {
		color: transparent;
		position: absolute;
		white-space: pre;
		cursor: text;
		transform-origin: 0% 0%;
	}
</style>
