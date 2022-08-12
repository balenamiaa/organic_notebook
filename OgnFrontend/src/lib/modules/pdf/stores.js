import { writable } from 'svelte/store';

export const pdfContextKey = Symbol();

export function createPdfDocumentContext(others) {
	const { subscribe, set, update } = writable(null);

	return {
		subscribe,
		set,
		update,
		...others,
	};
}
