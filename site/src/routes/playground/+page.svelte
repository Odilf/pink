<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import unwrap from 'ts-unwrap';

	import Evaluator from './Evaluator.svelte';
	import { vim } from '@replit/codemirror-vim';
	import CodeMirror from 'svelte-codemirror-editor';
	import exampleProgram from '$lib/../../../examples/propositional logic.pink?raw';
	import { barf } from 'thememirror';

	let selectedProgram = 0;

	const std = import.meta.glob('$lib/../../../standard_library/*.pink', { eager: true, as: 'raw' });
	const names = [...Object.keys(std).map((path) => unwrap(path.split('/').at(-1))), 'custom'];
	const programs = [...Object.values(std), exampleProgram];

	$: program = programs[selectedProgram];
</script>

<main class="h-screen w-full py-8 mx-auto max-w-3xl">
	<h1 class="font-bold text-5xl mb-4">Pink playground</h1>

	<Evaluator {program} />

	<Tabs.Root value="account" class="">
		<Tabs.List class="grid grid-rows-2 grid-cols-4">
			{#each names as name, i}
				<Tabs.Trigger class="flex-1" value={name} on:click={() => (selectedProgram = i)}
					>{name}</Tabs.Trigger
				>
			{/each}
		</Tabs.List>

		{#each programs as program, i}
		{@const editable = i === programs.length - 1}
			<Tabs.Content value={names[i]}>
				<div class="editor transition opacity-50 hover:opacity-100 focus-within:opacity-100 selection:opacity-100">
					<CodeMirror
						bind:value={program}
						{editable}
						extensions={editable ? [vim(), barf] : [barf]}
					/>
				</div>
			</Tabs.Content>
		{/each}
	</Tabs.Root>
</main>

<style lang="postcss">
	:global(.cm-editor) {
		overflow: hidden;
		@apply rounded;
	}
</style>
