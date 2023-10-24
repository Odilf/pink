<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import unwrap from 'ts-unwrap';

	import Evaluator from './Evaluator.svelte';
	import  { vim } from '@replit/codemirror-vim';
	import CodeMirror from 'svelte-codemirror-editor';
	import exampleProgram from '$lib/../../../examples/propositional logic.pink?raw';
	import { barf } from "thememirror"
	// data.worker.addFile('main', program);
	// data.worker.parseWithMainAs('main');

	// $: worker = data.worker;
	// $: status = worker.status;

	let expression = '';
	let selectedProgram = 0;

	const std = import.meta.glob('$lib/../../../standard_library/*.pink', { eager: true, as: 'raw' });
	const names = [...Object.keys(std).map((path) => unwrap(path.split('/').at(-1))), "custom"];
	const programs = [...Object.values(std), exampleProgram];

	$: program = programs[selectedProgram];
</script>

<main class="h-screen w-full py-8 mx-auto max-w-3xl">
	<h1 class="font-bold text-5xl mb-4">Pink playground</h1>

	<Evaluator {program} />

	<Tabs.Root value="account" class="">
		<Tabs.List class="grid grid-rows-2 grid-cols-4">
			{#each names as name, i}
				<Tabs.Trigger class="flex-1" value={name} on:click={() => selectedProgram = i}>{name}</Tabs.Trigger>
			{/each}
		</Tabs.List>

		{#each programs as program, i}
			<Tabs.Content value={names[i]}>
				<div class="editor">
					<CodeMirror bind:value={program} editable={i === programs.length - 1} extensions={[vim(), barf]}/>
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
