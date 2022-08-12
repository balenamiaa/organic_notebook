<script context="module">
	import workerSrc from 'pdfjs-dist/build/pdf.worker.min.js?url';
	import pdfViewStyle from 'pdfjs-dist/web/pdf_viewer.css';
	import { readable } from 'svelte/store';
	import * as pdfjs from 'pdfjs-dist';
	pdfjs.GlobalWorkerOptions.workerSrc = workerSrc;
	const pdfWorker = readable(null, (set) => {
		const worker = new pdfjs.PDFWorker();
		set(worker);
		return () => worker.destroy();
	});
</script>

<script>
	import { createEventDispatcher, setContext, onDestroy } from 'svelte';
	import { createPdfDocumentContext, pdfContextKey } from '../stores';
	import { documentViewerEvent } from '$lib/utils/events/documentViewerEvent';

	export let srcUrl;
	export let documentId;

	let loadingTask = null;
	const padding = 24;
	const pdfDocumentContext = createPdfDocumentContext({ padding });
	const dispatch = createEventDispatcher();

	setContext(pdfContextKey, pdfDocumentContext);

	$: if (srcUrl) loadPdf(srcUrl);

	async function loadPdf(url) {
		loadingTask = pdfjs.getDocument({ url, worker: $pdfWorker });
		try {
			const prevDoc = $pdfDocumentContext;
			$pdfDocumentContext = await loadingTask.promise;
			dispatch('loadSuccess', $pdfDocumentContext);

			prevDoc?.destroy();
			prevDoc?.cleanup();
		} catch (err) {}
	}

	onDestroy(() => {
		$pdfDocumentContext?.destroy();
		$pdfDocumentContext?.cleanup(false);
	});
</script>

<div
	id="root"
	data-document-id={documentId}
	style="padding: {padding}px;"
	use:documentViewerEvent
	on:selectionEnd
	on:selectionChange
>
	<slot />
</div>

<style>
	#root {
		background-color: rgba(237, 237, 240, 1);
		width: max-content;
		overflow: auto;
	}
</style>
