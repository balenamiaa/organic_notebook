import { writable } from 'svelte/store'
import { getDocuments } from './api'

export const documentsKey = Symbol()

export function createDocuments() {
	const { subscribe, set, update } = writable({ documents: [], actions: [] })

	return {
		subscribe,
		set,
		update,
		refresh: async () => {
			const documents = await (await getDocuments()).json()
			update((values) => {
				values.documents = documents.documents
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
