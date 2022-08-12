import { asJson } from '$lib/utils/api';
import { writable } from 'svelte/store';
import { getDocuments, getExtractedTexts } from './api';

export const DocumentsContextKey = Symbol();

export function createDocumentsContext() {
	const { subscribe, set, update } = writable({ documents: [], actions: [], extractedTexts: [] });

	return {
		subscribe,
		set,
		update,
		refresh: async () => {
			const documents = await asJson(getDocuments());
			update((context) => {
				context.documents = documents;
				return context;
			});
		},
		refreshExtractedTexts: async () => {
			const extractedTexts = await asJson(getExtractedTexts());
			update((context) => {
				context.extractedTexts = extractedTexts;
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
		getDocumentById: (documents, id) => {
			return documents.items.find((doc) => doc.id === id);
		},
	};
}
