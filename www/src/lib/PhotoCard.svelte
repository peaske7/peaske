<script lang="ts">
  import type { Orientation } from "$lib/types";

  export let orientation: Orientation = "landscape";
  export let name: string | undefined = undefined;
  export let desc: string | undefined = undefined;
</script>

<div class="mb-40 {orientation}">
  <slot />
  {#if name || desc}
    <p class="my-4">
      {#if !!name}
        <span>{name}</span>
        <br />
      {/if}
      <span>{desc}</span>
    </p>
  {/if}
</div>

<style lang="postcss">
  :global {
    .portrait img {
      max-height: 600px;
      width: auto;
    }

    .landscape img {
      max-width: 100%;
      height: auto;
    }
  }
</style>
