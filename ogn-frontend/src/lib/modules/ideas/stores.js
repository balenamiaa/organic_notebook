import { writable } from 'svelte/store'

export const ideasKey = Symbol()

export function createIdeas() {
	const { subscribe, set, update } = writable({})

	return {
		subscribe,
		set,
		update,
	}
}
