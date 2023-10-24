<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import exampleProgram from '$lib/../../../examples/propositional logic.pink?raw';
	import { getWorker, type Status } from '$lib';

	import Expression from './Expression.svelte';
	import Spinner from './Spinner.svelte';
	import { browser } from '$app/environment';

	let result: { expression: string; timeElapsed: number } | null = null;
	let status: Status = "idle"

	export let expression = '';
	export let program = exampleProgram;

	let worker: Awaited<ReturnType<typeof getWorker>> | null = null;

	async function evaluate(expression: string, program: string) {
		if (!browser) {
			return
		}

		if (worker) {
			worker.close();
		}

		worker = await getWorker();
		worker.addFile("main", program);
		worker.parseWithMainAs("main");

		worker.subscribe((expression, timeElapsed) => {
			result = {
				expression,
				timeElapsed
			};
		});

		worker.status.subscribe((s) => {
			status = s;
		});
		
		worker.evaluate(expression);
	}

	$: evaluate(expression, program);
</script>

<div class="flex relative">
	<Input
		type="text"
		placeholder="write an expression to evaluate (e.g.: 1 + 3)"
		class="w-full transition duration-500 font-mono bg-muted {status === "invalid" && "bg-destructive"}"
		bind:value={expression}
	/>

	<span class="absolute right-0 scale-[40%] -translate-y-5 translate-x-6">
		{#if status === "processing"}
			<Spinner />
		{/if}
	</span>
</div>

<div class="text-2xl font-bold pt-2 pb-4">
	<Expression expression={result?.expression ?? ""} timeElapsed={result?.timeElapsed} />
</div>
