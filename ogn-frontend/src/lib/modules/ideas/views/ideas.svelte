<script>
	let ideas = []
</script>

<div>
	<h2>Ideas</h2>
	<h3>List</h3>
	<ol>
		{#each ideas as idea}
			<li>{idea}</li>
		{/each}
	</ol>
</div>
