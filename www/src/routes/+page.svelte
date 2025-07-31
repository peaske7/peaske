<script lang="ts">
	const posts = [
		{
			title: 'Good Software',
			slug: 'good-software',
			date: 'August 1, 2025'
		}
	];
</script>

<svelte:head>
	<title>peaske</title>
</svelte:head>

<section>
	<p>
		I'm <a href="https://github.com/peaske7">peaske</a>, a software developer based in Tokyo.
	</p>
	<p>
		For the past few years, I worked on fintech services in Japan using Rust and Typescript, that
		handled transactions for millions of users.
	</p>
	<p>
		Now, I'm working on a startup in Tokyo with my good friend. We're building a new way to think
		about sales calls.
	</p>
	<p>I like taking <a href="/photos">pictures</a> when I travel as well.</p>
</section>

<div class="my-16 border-t border-gray-200"></div>

<section>
	<ul class="list-none p-0">
		{#each posts as post}
			<li class="mb-3">
				<a href="/blog/{post.slug}" class="no-underline hover:underline">{post.title}</a>
				<time class="ml-4 text-sm opacity-50">{post.date}</time>
			</li>
		{/each}
	</ul>
</section>

<style lang="postcss">
	@reference "../app.css";
</style>
