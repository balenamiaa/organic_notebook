import { writable } from 'svelte/store'

export const documentsKey = Symbol()

export function createDocuments() {
	const { subscribe, set, update } = writable({})

	return {
		subscribe,
		set,
		update,
	}
}
