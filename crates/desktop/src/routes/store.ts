import { invoke } from '@tauri-apps/api'
import { derived, writable, type Writable } from 'svelte/store'

export const savePath = writable<string>(
	localStorage.getItem('__save_path') || ''
)
savePath.subscribe(async value => {
	localStorage.setItem('__save_path', value)
})

export type SaveSlots = Array<{ name: string; level: number; death: number }>
export const saveSlots = derived<Writable<string>, SaveSlots>(
	savePath,
	($savePath, set) => {
		set([])
		invoke<SaveSlots>('load_save', { location: $savePath }).then(set)
	}
)

export const selectedSlot = writable<number>(
	parseInt(localStorage.getItem('__selected_slot') ?? '0') || 0
)

selectedSlot.subscribe(value => {
	localStorage.setItem('__selected_slot', value.toString())
})

export const selectedCharacter = derived(
	[saveSlots, selectedSlot],
	([$saveSlots, $selectedSlot]) => $saveSlots[$selectedSlot]
)

export const outputDirectory = writable<string>(
	localStorage.getItem('__output_dir') || ''
)
outputDirectory.subscribe(value => localStorage.setItem('__output_dir', value))

export const outputFilename = writable<string>(
	localStorage.getItem('__output_name') || 'count.txt'
)
outputFilename.subscribe(value => localStorage.setItem('__output_name', value))

export const outputFormat = writable<string>(
	localStorage.getItem('__output_format') || 'Deaths: {}'
)
outputFormat.subscribe(value => localStorage.setItem('__output_format', value))

export const countFrom = writable<number>(
	parseInt(localStorage.getItem('__count_from') ?? '0') || 0
)
countFrom.subscribe(value => localStorage.setItem('__count_from', value.toString()))
