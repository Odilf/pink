<script lang="ts">
	import { AsyncRuntime } from "$runtime"
	import { onMount } from "svelte";
	
	const runtime = AsyncRuntime.new();

	const stream = runtime.get_stream() as ReadableStream;
	const reader = stream.getReader();

	const program = `
	# Simple example for propositional logic
	# Does not resolve properly ambiguities
	# E.g.: it thinks that \`~false or true\` is both \`(~false) or true\` and \`~(false or true)\` 

	domain { true, false }
	reserve { not, ~, and, ^, or, V, ->, xor, nand, nor }
	use { }

	# Aliases
	not = ~;
	and = ^;
	or = V;

	# Not
	~true = false;
	~false = true;

	# And
	true ^ true = true;
	p ^ q = false;

	# Rest of them
	p V q = ~((~p) ^ (~q));
	p -> q = (~p) V q;
	p nand q = ~(p ^ q);
	p nor q = ~(p V q);
	`

	onMount(async () => {
		runtime.send_program("main", program);
		runtime.set_main_program("main");

		runtime.evaluate("true and false");

		// console.log(await reader.read());
		// console.log(await reader.read());
		// console.log(await reader.read());
		// console.log(await reader.read());
	})
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

hello
