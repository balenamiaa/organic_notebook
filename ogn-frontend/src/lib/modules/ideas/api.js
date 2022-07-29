import { contentType, getRequest, postRequest } from '$lib/utils/api.js'

export function getIdeas(page = 0, pageSize = 10) {
	return getRequest(`/ideas?page_num=${page}&page_size=${pageSize}`)
}

export function getIdeaEntry(id) {
	return getRequest(`/ideas/${id}`)
}

export function createIdea(data) {
	return postRequest(`/create_idea`, {
		body: JSON.stringify(data),
		headers: { ...contentType.json },
	})
}

export function createIdeaReference(data) {
	return postRequest(`/create_idea_ref`, {
		body: JSON.stringify(data),
		headers: { ...contentType.json },
	})
}
