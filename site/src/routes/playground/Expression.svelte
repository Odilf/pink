<script lang="ts">
	import { unified } from 'unified';
	import remarkRehype from 'remark-rehype';
	import remarkParse from 'remark-parse';
	import rehypeStringify from 'rehype-stringify';

	export let expression: string;
	export let timeElapsed: number | undefined;
</script>

<div class="relative">
	{#if expression}
		{#await unified()
			.use(remarkParse)
			.use(remarkRehype)
			.use(rehypeStringify)
			.process(expression) then html}
			{@html html}
		{/await}

		{#if timeElapsed}
			<span class="text-sm text-muted-foreground absolute top-0 right-0">(took {timeElapsed?.toFixed(2)}s) &nbsp;</span>
		{/if}
	{:else}
		<p>&nbsp</p>
	{/if}

</div>

<style lang="postcss">
	div > :global(p > em) {
		@apply text-primary not-italic;
	}
</style>
