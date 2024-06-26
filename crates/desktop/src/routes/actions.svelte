<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import {
		countFrom,
		outputDirectory,
		outputFilename,
		outputFormat,
		selectedCharacter,
	} from "./store";

	let start = false;
	let timer: number | null = null;

	$: {
		if (start) {
			timer = setInterval(() => {
				console.debug("write_save...");
				invoke("write_save", {
					death: $selectedCharacter.death,
					from: $countFrom,
					outdir: $outputDirectory,
					filename: $outputFilename,
					format: $outputFormat,
				});
			}, 1000);
		} else {
			if (timer) clearInterval(timer);
		}
	}

	function countFromZero() {
		if ($countFrom == 0) {
			countFrom.set($selectedCharacter.death);
		} else {
			countFrom.set(0);
		}
	}
</script>

<div class="grid grid-cols-2 gap-2">
	<button
		type="button"
		class="flex-shrink-0 px-2 py-2 text-sm font-semibold rounded shadow-sm bg-zinc-100 text-zinc-600 hover:bg-zinc-200"
		on:click={countFromZero}
	>
		{#if $countFrom == 0}
			Count from Zero
		{:else}
			Count everything
		{/if}
	</button>
	<button
		type="button"
		class="flex-shrink-0 px-2 py-2 text-sm font-semibold rounded shadow-sm bg-zinc-900 text-zinc-100 hover:bg-zinc-800"
		on:click={() => (start = !start)}
	>
		{start ? "Stop!!" : "Start!"}
	</button>
</div>
