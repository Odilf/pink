import readme from "$lib/../../../README.md?raw"

import remarkRehype from 'remark-rehype';
import remarkParse from 'remark-parse';
import rehypeStringify from 'rehype-stringify';
import { unified } from "unified";

export const prerender = true;

export async function load({}) {
	const html = await unified()
		.use(remarkParse)
		.use(remarkRehype)
		.use(rehypeStringify)
		.process(readme)

	return {
		html,
	}
}
