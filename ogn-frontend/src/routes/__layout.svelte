<script>
	import { createDocuments, documentsKey } from '$lib/modules/documents/stores'
	import { setContext } from 'svelte'

	setContext(documentsKey, {
		documents: createDocuments(),
	})
</script>

<slot />
