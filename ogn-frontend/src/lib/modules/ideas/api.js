import { getRequest } from '$lib/utils/api.js'

export function getIdeas() {
	return getRequest('/ideas')
}

export function getIdeaEntry(id) {
	return getRequest(`/ideas/${id}`)
}