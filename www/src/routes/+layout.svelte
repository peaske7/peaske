<script lang="ts">
	import '../app.css';
</script>

<div class="max-w-[600px] w-full mx-auto px-4 py-8 h-screen flex flex-col">
	<header class="mb-12">
		<nav class="space-x-4">
			<a href="/">peaske</a>
			<a href="/photos">photos</a>
		</nav>
	</header>
	<main class="flex-grow">
		<slot />
	</main>
	<footer class="mt-30 pt-8 pb-4 border-t border-gray-200">
		<p class="text-sm opacity-60">Great reads</p>
		<ul class="list-none p-0 mt-2 space-y-1">
			<li>
				<a href="https://grugbrain.dev" class="text-sm text-gray-400">The Grug Brained Developer</a>
			</li>
			<li>
				<a href="https://mcfunley.com/choose-boring-technology" class="text-sm text-gray-400">Choose Boring Technology</a>
			</li>
			<li>
				<a href="https://web.stanford.edu/~ouster/cgi-bin/aposd.php" class="text-sm text-gray-400">A Philosophy of Software Design</a>
			</li>
			<li>
				<a href="https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate" class="text-sm">Parse, Don't Validate</a>
			</li>
		</ul>
	</footer>
</div>

<style lang="postcss">
	@reference "../app.css";
</style>