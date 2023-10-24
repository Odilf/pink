<script lang="ts">
	import { Button } from '$lib/components/ui/button';

	export let data
</script>

<div class="w-screen max-w-xl mx-auto py-8">
	{@html data.html}

	<Button class="text-xl text-muted-foreground" href="https://github.com/Odilf/pink" variant="outline"> GitHub </Button>
</div>

<style lang="postcss">
	:global(h1) {
		@apply text-5xl font-bold mb-6;
	}

	:global(h2) {
		@apply text-4xl font-bold mb-5;
	}

	:global(h3) {
		@apply text-3xl font-bold mb-4;
	}

	:global(pre) {
		@apply bg-secondary text-secondary-foreground p-2 text-sm rounded mb-4 whitespace-break-spaces;
	}

	:global(p) {
		@apply mb-4;
	}
</style>
