<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	import { savePath } from "./store";

	async function selectFile() {
		const newPath = (await open({
			multiple: false,
			filters: [
				{
					name: "Elden Ring Save",
					extensions: ["sl2"],
				},
			],
		})) as string;
		if (newPath && newPath !== $savePath) {
			savePath.set(newPath);
		}
	}
</script>

<div class="space-y-1">
	<div class="text-sm">Select your Elden Ring save file to start</div>
	<div class="flex items-center gap-1">
		<input
			class="flex-1 px-2 py-1 text-xs rounded bg-zinc-100"
			type="text"
			readonly
			placeholder="It's should be in %APPDATA%\EldenRing"
			value={$savePath}
			on:click={selectFile}
		/>
		<button
			type="button"
			class="flex-shrink-0 px-2 py-1 text-xs font-semibold rounded shadow-sm bg-zinc-100 text-zinc-600 hover:bg-zinc-200"
			on:click={selectFile}
		>
			Browse
		</button>
	</div>
</div>
