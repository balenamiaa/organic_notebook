import { writable } from 'svelte/store'
import { getIdeas } from './api'

export const ideasKey = Symbol()

export function createIdeas() {
	const { subscribe, set, update } = writable({})

	return {
		subscribe,
		set,
		update,
		refresh: async () => {
			const ideas = await (await getIdeas()).json()
			update(() => ideas)
		},
	}
}
