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

We can measure speed: Time to first paint, latency, throughput. We can measure reliability: uptime, error rates, data consistency. But beauty? That's harder to quantify.

## What Makes Software Beautiful?

Beautiful software has clarity of purpose. It does one thing well. It respects the user's time and attention. The interface fades away, leaving only the task at hand.

Consider the command line. No animations, no gradients, just text on a background. Yet developers find it beautiful. Why? Because it's honest. What you type is what happens. The feedback is immediate and unambiguous.

## The Cost of Complexity

Every feature has a cost. Not just in development time, but in cognitive load. Each button, each option, each configuration setting asks something of the user.

Good software says no more often than it says yes. It chooses defaults carefully. It makes the common case simple and the complex case possible.

## Fast is Beautiful

Speed is a feature. A fast application respects the user. It says: "Your time matters. Your flow state is sacred."

Users might not consciously notice when software is fast, but they always notice when it's slow. Every unnecessary millisecond is a small betrayal of trust.

## Conclusion

Good software is opinionated. It has a point of view about how things should work. It doesn't try to be everything to everyone.

Build software that you would want to use. Make it fast. Make it reliable. Make it beautiful.

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