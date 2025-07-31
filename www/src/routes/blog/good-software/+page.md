<script context="module">
  export const metadata = {
    title: 'Good Software',
    date: 'August 1, 2025'
  };
</script>

<svelte:head>

  <title>{metadata.title} | peaske</title>
</svelte:head>

# {metadata.title}

<time class="text-sm opacity-50">{metadata.date}</time>

Good software is fast, reliable, and beautiful.

<!-- We can measure speed by tracking api latency, page load performance, p99.
We can measure reliability: uptime, error rates, data consistency.
Beauty is harder to measure, but easier to perceive. -->

<style>
  h1 {
    margin-bottom: 0.5rem;
  }
  
  h2 {
    margin-top: 2rem;
    margin-bottom: 1rem;
    font-size: 1.25rem;
    font-weight: normal;
  }
  
  p {
    margin: 1rem 0;
  }
</style>
