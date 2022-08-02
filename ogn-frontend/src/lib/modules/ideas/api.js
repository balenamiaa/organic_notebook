import { contentType, deleteRequest, getRequest, postRequest } from '$lib/utils/api.js'

export function getIdeas(page = 0, pageSize = 100) {
	return getRequest(`/ideas?page_num=${page}&page_size=${pageSize}`)
}

export function getIdeaEntry(id) {
	return getRequest(`/ideas/${id}`)
}

export function createIdea(data) {
	return postRequest(`/ideas`, {
		body: JSON.stringify(data),
		headers: { ...contentType.json },
	})
}

export function createIdeaReference(data) {
	return postRequest(`/idea_refs`, {
		body: JSON.stringify(data),
		headers: { ...contentType.json },
	})
}

export function deleteIdea(ideaId) {
	return deleteRequest(`/ideas/${ideaId}`)
}

export function deleteIdeaRef(ideaRefId) {
	return deleteRequest(`/idea_refs/${ideaRefId}`)

}

export function getIdeaRefs(ideaId, page = 0, pageSize = 10) {
	return getRequest(`/idea_refs_for_idea/${ideaId}?page_num=${page}&page_size=${pageSize}`)
}