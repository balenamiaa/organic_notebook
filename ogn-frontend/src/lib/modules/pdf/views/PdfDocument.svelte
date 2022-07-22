<script context="module">
	import workerSrc from 'pdfjs-dist/build/pdf.worker.min.js?url'
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
	import { PdfKey } from '../stores'

	export let srcUrl

	let loadingTask = null
	const pdfDocument = writable(null)
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

<slot />
