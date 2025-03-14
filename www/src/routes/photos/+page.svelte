<script lang="ts">
	const landscape = { width: 1067, height: 1600 };
	const portrait = { width: 1600, height: 1067 };

	// Define image dimensions to prevent layout shifts
	const imageDimensions = {
		"a6400-03871": portrait, // Portrait
		"a6400-03307": portrait, // Portrait
		"a6400-03077": portrait, // Portrait
		"a6400-02423": landscape, // Landscape
		"a6400-01926": portrait, // Portrait
		"a6400-01621": landscape, // Landscape
		"a6400-01867": landscape, // Landscape
		"a6400-01715": landscape, // Landscape
		"a6400-01439": portrait, // Portrait
		"a6400-00985": landscape, // Landscape
	};
</script>

<svelte:head>
	<title>peaske | photos</title>
	<!-- Preload critical images -->
	<link
		rel="preload"
		href="/static/assets/a6400-03871.webp?quality=85"
		as="image"
		type="image/webp"
		fetchpriority="high"
	/>
	<link
		rel="preload"
		href="/static/assets/a6400-03307.webp?quality=85"
		as="image"
		type="image/webp"
	/>
	<link
		rel="preload"
		href="/static/assets/a6400-03077.webp?quality=85"
		as="image"
		type="image/webp"
	/>
	<link rel="dns-prefetch" href="/" />
	<link rel="preconnect" href="/" />
	<meta content="DPR, Width, Viewport-Width, ECT, RTT" />
	<meta name="image-compression" content="webp, quality=85, resize=fit" />
</svelte:head>

<h2>Photos</h2>
<p>Here are some of the photos I've taken.</p>

<p>
	<a href="/">Back</a>
</p>

<section class="mt-20">
	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-03871.webp?quality=85"
			alt="Odawara Castle"
			fetchpriority="high"
			decoding="sync"
			sizes="(max-width: 768px) 100vw, 600px"
			style="background-size: cover;"
		/>
		<p class="my-4">
			<span>小田原城 - Odawara Castle</span>
		</p>
	</div>

	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-03307.webp?quality=85"
			alt="Kamakura"
			fetchpriority="high"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
			style="background-size: cover;"
		/>
		<p class="my-4">
			<span>鎌倉 - Kamakura</span>
		</p>
	</div>

	<div class="mb-40 portrait">
		<enhanced:img
			src="/static/assets/a6400-03077.webp?quality=85"
			alt="Engakuji Temple"
			fetchpriority="high"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>円覚寺 - Engakuji Temple</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-02423.webp?quality=85"
			alt="Yokohama"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>横浜 - Yokohama</span>
		</p>
	</div>

	<div class="mb-40 portrait content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01926.webp?quality=85"
			alt="Kenrokuen Garden"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>兼六園 - Kenrokuen Garden</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01621.webp?quality=85"
			alt="Chidorigafuchi Moat"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>千鳥ヶ淵 - Chidorigafuchi Moat</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01867.webp?quality=85"
			alt="21st Century Museum of Contemporary Art"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>金沢２１世紀美術館 - 21st Century Museum of Contemporary Art</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01715.webp?quality=85"
			alt="Coast of Tateyama, Chiba"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>館山の海岸 - Coast of Tateyama, Chiba</span>
		</p>
	</div>

	<div class="mb-40 portrait content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-01439.webp?quality=85"
			alt="Atami Cherry Blossoms"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 600px"
		/>
		<p class="my-4">
			<span>熱海桜 - Atami Cherry Blossoms</span>
		</p>
	</div>

	<div class="mb-40 landscape content-visibility-auto">
		<enhanced:img
			src="/static/assets/a6400-00985.webp?quality=85"
			alt="Kofukuji Temple"
			loading="lazy"
			decoding="async"
			sizes="(max-width: 768px) 100vw, 650px"
		/>
		<p class="my-4">
			<span>興福寺 - Kofukuji Temple</span>
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
