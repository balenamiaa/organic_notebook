<script>
	import { createDocuments, documentsKey } from '$lib/modules/documents/stores'
	import { createIdeas, ideasKey } from '$lib/modules/ideas/stores'
	import { setContext } from 'svelte'

	const documents = createDocuments()
	setContext(documentsKey, {
		documents,
	})
	const ideas = createIdeas()
	setContext(ideasKey, {
		ideas,
	})
</script>

<slot />
