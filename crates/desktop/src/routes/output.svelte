<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	import {
		outputDirectory,
		outputFilename,
		selectedCharacter,
		outputFormat,
		countFrom,
	} from "./store";

	async function selectOutput() {
		const newPath = (await open({
			multiple: false,
			directory: true,
		})) as string;
		if (newPath && newPath !== $outputDirectory) {
			outputDirectory.set(newPath);
		}
	}
</script>

<div class="space-y-1">
	<div class="text-sm">Output:</div>
	<div class="grid grid-cols-2 gap-2">
		<div class="space-y-0.5">
			<label class="block text-sm" for="output_dir">Save location</label>
			<input
				id="output_dir"
				class="flex-1 w-full px-2 py-1 text-xs rounded bg-zinc-100"
				type="text"
				placeholder="Save location"
				bind:value={$outputDirectory}
				on:click={selectOutput}
			/>
		</div>
		<div class="space-y-0.5">
			<label class="block text-sm" for="filename">Filename</label>
			<input
				id="filename"
				class="flex-1 w-full px-2 py-1 text-xs rounded bg-zinc-100"
				type="text"
				placeholder="count.txt"
				bind:value={$outputFilename}
			/>
		</div>
	</div>
	<div class="grid grid-cols-2 gap-x-2 gap-y-0.5">
		<div class="space-y-0.5">
			<label class="block text-sm" for="format">Format</label>
			<textarea
				id="format"
				class="flex-1 block w-full h-24 px-2 py-1 text-xs rounded resize-none bg-zinc-100"
				placeholder="Output file format"
				bind:value={$outputFormat}
			/>
		</div>
		<div class="space-y-0.5 flex flex-col">
			<span class="block text-sm">Preview</span>
			<textarea
				class="flex-1 w-full h-full px-2 py-1 text-xs rounded resize-none bg-zinc-100"
				readonly
				value={$outputFormat.replace(
					"{}",
					(($selectedCharacter?.death || 0) - $countFrom).toString(),
				)}
				placeholder="Preview your output"
			/>
		</div>
		<div class="col-span-2 text-sm">
			The symbols <strong><code>{"{}"}</code></strong> will be replace by your
			death.
		</div>
	</div>
</div>
