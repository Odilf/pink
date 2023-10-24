<script>
	import { Button } from '$lib/components/ui/button';
</script>

<main class="w-screen h-screen grid place-content-center">
	<h1 class="text-8xl font-black text-center">Pink</h1>
	<p class="pb-8 text-xl font-light">
		A very minimal replacement based DSL, intended for math experimentation
	</p>

	<div class="flex w-full max-w-sm mx-auto">
		<Button variant="default" class="flex-1 text-xl font-bold" href="/playground">
			Playground
		</Button>
		<Button variant="link" class="flex-1 text-xl font-bold" href="/docs">Documentation</Button>
	</div>

	<Button variant="ghost" class="w-fit mx-auto mt-8" href="https://github.com/Odilf/pink"> GitHub </Button>
	<Button variant="ghost" class="w-fit mx-auto mt-1" href="https://odilf.com"> More from Odilf </Button>
</main>
