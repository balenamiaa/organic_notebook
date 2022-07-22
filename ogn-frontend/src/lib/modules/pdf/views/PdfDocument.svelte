<script context="module">
	import workerSrc from 'pdfjs-dist/build/pdf.worker.min.js?url'
	import pdfViewStyle from 'pdfjs-dist/web/pdf_viewer.css'
	import { readable, writable } from 'svelte/store'
	import * as pdfjs from 'pdfjs-dist'
	pdfjs.GlobalWorkerOptions.workerSrc = workerSrc
	const pdfWorker = readable(null, (set) => {
		const worker = new pdfjs.PDFWorker()
		set(worker)
		return () => worker.destroy()
	})
</script>

<script>
	import { createEventDispatcher, setContext, onDestroy } from 'svelte'
	import { createPdfDocument, PdfKey } from '../stores'

	export let srcUrl

	let loadingTask = null
	const padding = 24
	const pdfDocument = createPdfDocument({ padding })
	const dispatch = createEventDispatcher()

	setContext(PdfKey, pdfDocument)

	$: if (srcUrl) loadPdf(srcUrl)

	async function loadPdf(url) {
		loadingTask = pdfjs.getDocument({ url, worker: $pdfWorker })
		try {
			const prevDoc = $pdfDocument
			$pdfDocument = await loadingTask.promise
			dispatch('loadSuccess', $pdfDocument)

			prevDoc?.destroy()
			prevDoc?.cleanup()
		} catch (err) {}
	}

	onDestroy(() => {
		$pdfDocument?.destroy()
		$pdfDocument?.cleanup(false)
	})
</script>

<div id="root" style="padding: {padding}px;">
	<slot />
</div>

<style>
	#root {
		background-color: rgba(237, 237, 240, 1);
		width: max-content;
		overflow: auto;
	}
</style>
