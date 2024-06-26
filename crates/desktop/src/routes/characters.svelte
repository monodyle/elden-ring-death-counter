<script lang="ts">
	import { saveSlots, selectedCharacter, selectedSlot } from "./store";

	let selectedSlotValue = $selectedSlot;
	$: selectedSlot.set(selectedSlotValue);
</script>

<div class="space-y-1">
	<div class="text-sm">Select character to continue</div>
	<select
		name="location"
		class="block w-full px-2 py-1 pr-10 mt-2 text-sm border-0 rounded-md text-zinc-900 ring-1 ring-inset ring-zinc-300 focus:ring-2 focus:ring-zinc-600"
		bind:value={selectedSlotValue}
	>
		{#each $saveSlots as slot, index}
			<option
				value={index.toString()}
				selected={selectedSlotValue == index}
			>
				{slot.name} - Level: {slot.level}
			</option>
		{/each}
	</select>
	{#if $selectedCharacter}
		<div class="font-medium text-right">
			{$selectedCharacter.name} died total
			<span class="font-medium text-red-700 text-">
				{$selectedCharacter.death}
			</span>
			times
		</div>
	{/if}
</div>
