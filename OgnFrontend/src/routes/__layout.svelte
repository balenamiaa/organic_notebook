<script>
	import { createDocumentsContext, DocumentsContextKey } from '$lib/modules/documents/stores';
	import { createIdeasContext, IdeasContextKey } from '$lib/modules/ideas/stores';
	import { setContext } from 'svelte';

	const documentsContext = createDocumentsContext();
	setContext(DocumentsContextKey, {
		documentsContext: documentsContext,
	});
	const ideasContext = createIdeasContext();
	setContext(IdeasContextKey, {
		ideasContext: ideasContext,
	});
</script>

<slot />
