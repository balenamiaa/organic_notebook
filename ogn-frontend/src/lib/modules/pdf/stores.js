import { writable } from 'svelte/store'

export const PdfKey = Symbol()

export function createPdfDocument(others) {
	const { subscribe, set, update } = writable(null)

	return {
		subscribe,
		set,
		update,
		...others,
	}
}
