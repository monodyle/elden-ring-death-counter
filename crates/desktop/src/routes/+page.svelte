<script lang="ts">
	import Actions from "./actions.svelte";
import Characters from "./characters.svelte";
	import Output from "./output.svelte";
	import Selector from "./selector.svelte";
</script>

<div class="flex flex-col w-screen h-screen gap-2 p-4">
	<h1 class="text-2xl font-semibold text-center">Death Counter</h1>
	<Selector />
	<Characters />
	<Output />
	<Actions />
</div>
