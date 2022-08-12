import { asJson } from '$lib/utils/api';
import { writable } from 'svelte/store';
import { getIdeas } from './api';

export const IdeasContextKey = Symbol();

export function createIdeasContext() {
	const { subscribe, set, update } = writable({ ideas: [], actions: [] });

	return {
		subscribe,
		set,
		update,
		refresh: async () => {
			const ideas = await asJson(getIdeas());
			update((context) => {
				context.ideas = ideas;
				return context;
			});
		},
		pushAction: (action) => {
			update((context) => {
				context.actions.push(action);
				return context;
			});
		},
		removeAction: (index) => {
			update((context) => {
				context.actions.splice(index, 1);
				return context;
			});
		},
	};
}
