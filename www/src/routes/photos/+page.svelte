<script lang="ts" context="module">
</script>

<svelte:head>
	<title>peaske | photos</title>
</svelte:head>

<h2>Photos</h2>

<section class="mt-8">
	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-04554.jpg?quality=85"
			alt="Keelung"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Keelung, Taiwan</p>
	</div>

	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-04481.jpg?quality=85"
			alt="Keelung"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Keelung, Taiwan</p>
	</div>

	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-04422.jpg?quality=85"
			alt="Dihua Street"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Dihua Street, Taipei</p>
	</div>

	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-03871.jpg?quality=85"
			alt="Odawara Castle"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Odawara Castle, Japan</p>
	</div>

	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-03307.jpg?quality=85"
			alt="Kamakura"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Kamakura, Japan</p>
	</div>

	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-03077.jpg?quality=85"
			alt="Engakuji Temple"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Engakuji Temple, Kamakura</p>
	</div>

	<div class="mb-16 landscape">
		<enhanced:img
			src="/static/assets/a6400-02423.jpg?quality=85"
			alt="Yokohama"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Yokohama, Japan</p>
	</div>

	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-01926.jpg?quality=85"
			alt="Kenrokuen Garden"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Kenrokuen Garden, Kanazawa</p>
	</div>

	<div class="mb-16 landscape">
		<enhanced:img
			src="/static/assets/a6400-01621.jpg?quality=85"
			alt="Chidorigafuchi Moat"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Chidorigafuchi, Tokyo</p>
	</div>

	<div class="mb-16 landscape">
		<enhanced:img
			src="/static/assets/a6400-01867.jpg?quality=85"
			alt="21st Century Museum of Contemporary Art"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">21st Century Museum, Kanazawa</p>
	</div>

	<div class="mb-16 landscape">
		<enhanced:img
			src="/static/assets/a6400-01715.jpg?quality=85"
			alt="Coast of Tateyama, Chiba"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Tateyama Coast, Chiba</p>
	</div>

	<div class="mb-16 portrait">
		<enhanced:img
			src="/static/assets/a6400-01439.jpg?quality=85"
			alt="Atami Cherry Blossoms"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Cherry Blossoms, Atami</p>
	</div>

	<div class="mb-16 landscape">
		<enhanced:img
			src="/static/assets/a6400-00985.jpg?quality=85"
			alt="Kofukuji Temple"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="mt-2 text-sm opacity-50 text-center">Kofukuji Temple, Nara</p>
	</div>
</section>

<style lang="postcss">
	@reference "../../app.css";

	:global(.portrait img) {
		max-height: 600px;
		width: auto;
	}

	:global(.landscape img) {
		max-width: 100%;
		height: auto;
	}


	:global(enhanced\:img) {
		background-position: center;
		background-repeat: no-repeat;
		transition: opacity 0.3s ease;
		cursor: pointer;
	}
	
	:global(enhanced\:img:hover) {
		opacity: 0.9;
	}
</style>
