import { writable } from 'svelte/store'
import { getDocuments, getExtractedTexts } from './api'

export const documentsKey = Symbol()

export function createDocuments() {
	const { subscribe, set, update } = writable({ documents: [], actions: [], extractedTexts: [] })

	return {
		subscribe,
		set,
		update,
		refresh: async () => {
			const documents = await getDocuments().then(response => response.json())
			update((values) => {
				values.documents = documents.documents
				return values
			})
		},
		refreshExtractedTexts: async () => {
			const json = await getExtractedTexts().then(response => response.json())
			update((values) => {
				values.extractedTexts = json.extracted_texts
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
		},
		getDocumentById: (documents, id) => {
			return documents.find(doc => doc.id === id)
		}
	}
}
