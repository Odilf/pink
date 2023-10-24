<script lang="ts">
	import { unified } from 'unified';
	import remarkRehype from 'remark-rehype';
	import remarkParse from 'remark-parse';
	import rehypeStringify from 'rehype-stringify';

	export let expression: string;
</script>

<span>

	{#await unified()
	.use(remarkParse)
	.use(remarkRehype)
	.use(rehypeStringify)
	.process(expression) then html}
	{@html html}
	{/await}
</span>
	
<style lang="postcss">
	span > :global(p > em) {
		@apply text-primary not-italic;
	}
</style>
