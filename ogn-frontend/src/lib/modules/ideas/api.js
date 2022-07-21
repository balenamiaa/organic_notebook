import { getRequest } from '$lib/utils/api.js'

export function getIdeas() {
	return getRequest('/ideas')
}

export function getDocumentEntry(id) {
	return getRequest(`/ideas/${id}`)
}
