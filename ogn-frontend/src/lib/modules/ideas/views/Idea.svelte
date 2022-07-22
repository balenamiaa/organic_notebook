<script>
	import { getContext } from 'svelte'
	import { createIdea, getIdeas } from '../api.js'
	import { ideasKey } from '../stores.js'

	const { ideas } = getContext(ideasKey)

	refreshIdeas()

	async function onSubmit(event) {
		try {
			const ideaLabel = event.target.elements['idea_label'].value
			const response = await createIdea({ label: ideaLabel })

			if (response.status === 200) {
				refreshIdeas()
			}
		} catch (err) {}
	}
	async function refreshIdeas() {
		try {
			$ideas = await (await getIdeas()).json()
		} catch (err) {}
	}
</script>

<div>
	<h2>Ideas</h2>
	<form on:submit|preventDefault={onSubmit}>
		Create Idea
		<input type="text" name="idea_label" placeholder="Idea label" />
		<div>
			<button>Submit</button>
		</div>
	</form>
	<h3>List</h3>
	{#if $ideas.ideas}
		<ol>
			{#each $ideas.ideas as idea}
				<li>
					{idea.label}
				</li>
			{/each}
		</ol>
	{/if}
</div>
