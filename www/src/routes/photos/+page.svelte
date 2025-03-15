<script lang="ts" context="module">
</script>

<svelte:head>
	<title>peaske | photos</title>
	<link
		rel="preload"
		href="/assets/a6400-03871.jpg?quality=85"
		as="image"
		type="image/jpeg"
		fetchpriority="high"
	/>
	<link
		rel="preload"
		href="/assets/a6400-03307.jpg?quality=85"
		as="image"
		type="image/jpeg"
	/>
	<link
		rel="preload"
		href="/assets/a6400-03077.jpg?quality=85"
		as="image"
		type="image/jpeg"
	/>
	<link rel="dns-prefetch" href="/" />
	<link rel="preconnect" href="/" />
	<meta content="DPR, Width, Viewport-Width, ECT, RTT" />
	<meta name="image-compression" content="jpg, quality=85, resize=fit" />
</svelte:head>

<h2>Photos</h2>
<p>Here are some of the photos I've taken.</p>

<p>
	<a href="/">Back</a>
</p>

<section class="mt-20">
	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-04554.jpg?quality=85"
			alt="Keelung"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
			style="background-size: cover;"
		/>
		<p class="my-4">
			<span>Keelung</span>
		</p>
	</div>

	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-04481.jpg?quality=85"
			alt="Keelung"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>Keelung</span>
		</p>
	</div>

	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-04422.jpg?quality=85"
			alt="Dihua Street"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>Dihua Street</span>
		</p>
	</div>

	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-03871.jpg?quality=85"
			alt="Odawara Castle"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
			style="background-size: cover;"
		/>
		<p class="my-4">
			<span>Odawara Castle</span>
		</p>
	</div>

	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-03307.jpg?quality=85"
			alt="Kamakura"
			fetchpriority="high"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
			style="background-size: cover;"
		/>
		<p class="my-4">
			<span>Kamakura</span>
		</p>
	</div>

	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-03077.jpg?quality=85"
			alt="Engakuji Temple"
			fetchpriority="high"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>Engakuji Temple</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-02423.jpg?quality=85"
			alt="Yokohama"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>Yokohama</span>
		</p>
	</div>

	<div class="mb-40 portrait content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01926.jpg?quality=85"
			alt="Kenrokuen Garden"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>Kenrokuen Garden</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01621.jpg?quality=85"
			alt="Chidorigafuchi Moat"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>Chidorigafuchi Moat</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01867.jpg?quality=85"
			alt="21st Century Museum of Contemporary Art"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>21st Century Museum of Contemporary Art</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01715.jpg?quality=85"
			alt="Coast of Tateyama, Chiba"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>Coast of Tateyama, Chiba</span>
		</p>
	</div>

	<div class="mb-40 portrait content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01439.jpg?quality=85"
			alt="Atami Cherry Blossoms"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>Atami Cherry Blossoms</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-00985.jpg?quality=85"
			alt="Kofukuji Temple"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>Kofukuji Temple</span>
		</p>
	</div>
</section>

<style lang="postcss">
	p {
		@apply my-4;
	}

	:global(.portrait img) {
		max-height: 600px;
		width: auto;
	}

	:global(.landscape img) {
		max-width: 100%;
		height: auto;
	}

	.content-visibility-auto {
		content-visibility: auto;
		contain-intrinsic-size: 1px 1000px;
	}

	:global(enhanced\:img) {
		background-position: center;
		background-repeat: no-repeat;
		transition: opacity 0.5s ease-in-out;
	}

	/* Add fade-in animation for images */
	:global(enhanced\:img.loaded) {
		animation: fadeIn 0.5s;
	}

	@keyframes fadeIn {
		from {
			opacity: 0.5;
		}
		to {
			opacity: 1;
		}
	}
</style>
