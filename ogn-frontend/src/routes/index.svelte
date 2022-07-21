<script context="module">
	/** @type {import('@sveltejs/kit').Handle} */
	export async function handle({ event, resolve }) {
		const response = await resolve(event, {
			ssr: false,
		})
		return response
	}
</script>

<script>
	import Documents from '$lib/modules/documents/views/documents.svelte'
	import Ideas from '$lib/modules/ideas/views/ideas.svelte'
</script>

<h1>Organic Notebook</h1>

<Documents />

<Ideas />
