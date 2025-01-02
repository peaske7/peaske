<script lang="ts">
	import "../app.css";
	let { children } = $props();
</script>

<div class="max-w-[650px] w-full my-4 mx-auto p-4">
	{@render children()}
</div>
