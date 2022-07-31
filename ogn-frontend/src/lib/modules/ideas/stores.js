import { writable } from 'svelte/store'
import { getIdeas } from './api'

export const ideasKey = Symbol()

export function createIdeas() {
	const { subscribe, set, update } = writable({ ideas: [], actions: [] })

	return {
		subscribe,
		set,
		update,
		refresh: async () => {
			const ideas = await getIdeas().then(response => response.json())
			update((values) => {
				values.ideas = ideas.ideas
				return values
			})
		},
		pushAction: (action) => {
			update((values) => {
				values.actions.push(action)
				return values
			})
		},
		removeAction: (index) => {
			update((values) => {
				values.actions.splice(index, 1)
				return values
			})
		}
	}
}
