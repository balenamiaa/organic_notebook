import {
	contentType,
	deleteRequest,
	getPaginatedRequest,
	getRequest,
	postRequest,
} from '$lib/utils/api.js';

export function getIdeas(options = {}) {
	return getPaginatedRequest('/ideas', options);
}

export function getIdeaEntry(id) {
	return getRequest(`/ideas/${id}`);
}

export function createIdea(data) {
	return postRequest(`/ideas`, {
		body: JSON.stringify(data),
		headers: { ...contentType.json },
	});
}

export function createIdeaReference(data) {
	return postRequest(`/idea_refs`, {
		body: JSON.stringify(data),
		headers: { ...contentType.json },
	});
}

export function deleteIdea(ideaId) {
	return deleteRequest(`/ideas/${ideaId}`);
}

export function deleteIdeaRef(ideaRefId) {
	return deleteRequest(`/idea_refs/${ideaRefId}`);
}

export function deleteIdeaRefsForIdea(ideaId) {
	return deleteRequest(`/idea_refs/ideas/${ideaId}`);
}

export function getIdeaRefs(options = {}) {
	return getPaginatedRequest(`/idea_refs`, options);
}

export function getIdeaRefsForIdea(ideaId, options = {}) {
	return getPaginatedRequest(`/idea_refs/ideas/${ideaId}`, options);
}
