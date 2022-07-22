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
		refreshPage()
	}
	$: if (page && viewport) {
		tick().then(() => {
			renderPage(page, viewport).then(() => renderText(page, viewport))
		})
	}

	async function refreshPage() {
		try {
			page = await $pdf.getPage(pageNumber)
			// To make page fit the screen and have consistent width
			const scale =
				(document.documentElement.clientWidth - pdf.padding * 2) / page.getViewport(1.0).viewBox[2]

			viewport = page.getViewport({ scale: scale })
		} catch (err) {}
	}
	async function renderPage(page, viewport) {
		if (pageRenderTask?._internalRenderTask.running) pageRenderTask.cancel()
		const context = canvas.getContext('2d')

		const outputScale = window.devicePixelRatio || 1
		const width = viewport.width
		const height = viewport.height
		canvas.width = Math.floor(width * outputScale)
		canvas.height = Math.floor(height * outputScale)
		canvas.style.width = Math.floor(width) + 'px'
		canvas.style.height = Math.floor(height) + 'px'

		var transform = outputScale !== 1 ? [outputScale, 0, 0, outputScale, 0, 0] : null

		const renderContext = {
			canvasContext: context,
			viewport: viewport,
			transform,
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

	<div bind:this={textContainer} class="textLayer" />
</div>

<style>
	#root {
		position: relative;
	}
</style>
