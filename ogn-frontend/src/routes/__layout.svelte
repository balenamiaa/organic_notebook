<script>
	import { createDocuments, documentsKey } from '$lib/modules/documents/stores'
	import { setContext } from 'svelte'

	const documents = createDocuments()
	setContext(documentsKey, {
		documents,
	})
</script>

<slot />
