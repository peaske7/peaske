<script>
	export let title = '';
	export let date = '';
</script>

<svelte:head>
	<title>{title} | peaske</title>
</svelte:head>

<article>
	<h1>{title}</h1>
	<time class="text-sm opacity-50">{date}</time>
	<div class="prose mt-8">
		<slot />
	</div>
</article>

<style lang="postcss">
	@reference "../app.css";

	:global(.prose) {
		max-width: none;
	}

	:global(.prose h2) {
		@apply text-xl mt-8 mb-4;
		font-weight: normal;
	}

	:global(.prose h3) {
		@apply text-lg mt-6 mb-3;
		font-weight: normal;
	}

	:global(.prose p) {
		@apply my-4;
	}

	:global(.prose pre) {
		@apply bg-gray-100 p-4 rounded overflow-x-auto my-4;
	}

	:global(.prose code) {
		@apply bg-gray-100 px-1 rounded text-sm;
	}

	:global(.prose blockquote) {
		@apply border-l-2 border-gray-300 pl-4 my-4 italic;
	}

	:global(.prose ul) {
		@apply list-disc list-inside my-4;
	}

	:global(.prose ol) {
		@apply list-decimal list-inside my-4;
	}
</style>
