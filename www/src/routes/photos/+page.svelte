<script lang="ts">
	import { PhotoCard } from "$lib";
</script>

<svelte:head>
	<title>peaske | photos</title>
</svelte:head>

<h2>Photos</h2>
<p>Here are some of the photos I've taken.</p>

<p>
	<a href="/">Back</a>
</p>

<section class="mt-20">
	<PhotoCard orientation="portrait" name="小田原城 - Odawara Castle">
		<enhanced:img src="/static/assets/a6400-03871.jpg" alt="Odawara Castle" />
	</PhotoCard>

	<PhotoCard orientation="portrait" name="鎌倉 - Kamakura">
		<enhanced:img src="/static/assets/a6400-03307.jpg" alt="Kamakura" />
	</PhotoCard>

	<PhotoCard orientation="portrait" name="円覚寺 - Engakuji Temple">
		<enhanced:img src="/static/assets/a6400-03077.jpg" alt="Engakuji Temple" />
	</PhotoCard>

	<PhotoCard name="横浜 - Yokohama">
		<enhanced:img src="/static/assets/a6400-02423.jpg" alt="Yokohama" />
	</PhotoCard>

	<PhotoCard orientation="portrait" name="兼六園 - Kenrokuen Garden">
		<enhanced:img src="/static/assets/a6400-01926.jpg" alt="Kenrokuen Garden" />
	</PhotoCard>

	<PhotoCard name="千鳥ヶ淵 - Chidorigafuchi Moat">
		<enhanced:img
			src="/static/assets/a6400-01621.jpg"
			alt="Chidorigafuchi Moat"
		/>
	</PhotoCard>

	<PhotoCard
		name="金沢２１世紀美術館 - 21st Century Museum of Contemporary Art"
	>
		<enhanced:img
			src="/static/assets/a6400-01867.jpg"
			alt="21st Century Museum of Contemporary Art"
		/>
	</PhotoCard>

	<PhotoCard name="館山の海岸 - Coast of Tateyama, Chiba">
		<enhanced:img
			src="/static/assets/a6400-01715.jpg"
			alt="Coast of Tateyama, Chiba"
		/>
	</PhotoCard>

	<PhotoCard orientation="portrait" name="熱海桜 - Atami Cherry Blossoms">
		<enhanced:img
			src="/static/assets/a6400-01439.jpg"
			alt="Atami Cherry Blossoms"
		/>
	</PhotoCard>

	<PhotoCard name="興福寺 - Kofukuji Temple">
		<enhanced:img src="/static/assets/a6400-00985.jpg" alt="Kofukuji Temple" />
	</PhotoCard>
</section>

<style lang="postcss">
	p {
		@apply my-4;
	}
</style>
