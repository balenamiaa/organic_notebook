import {
	deleteRequest,
	fetchFromBase,
	getPaginatedRequest,
	getRequest,
	postRequest,
} from '$lib/utils/api.js';

export function getDocuments(options = {}) {
	return getPaginatedRequest('/documents', options);
}

export function getDocumentEntry(id) {
	return getRequest(`/documents/${id}`);
}

export function uploadDocument(files) {
	const formData = new FormData();
	for (let i = 0; i < files.length; i++) {
		formData.append(files[i].name.replace(/\.[^/.]+$/, ''), files[i]);
	}
	return postRequest('/documents', { body: formData });
}

export function getDocumentFile(document) {
	return fetchFromBase(`/host/static/${document.id}.${document.filetype}`);
}

export function deleteDocument(documentId) {
	return deleteRequest(`/documents/${documentId}`);
}

export function getExtractedTexts(options = {}) {
	return getPaginatedRequest(`/extracted_texts`, options);
}

export function getDocumentsExtractedText(documentIds) {
	var ids = '';
	documentIds.forEach((element) => {
		ids += element;
		ids += ' ';
	});

	return getRequest('/extracted_texts/documents', {
		body: ids,
	});
}

export function extractDocumentText(documentId) {
	return postRequest(`/extracted_texts/documents/${documentId}`).then(async (response) => {
		if (response.status == 409) {
			// document already has texts for it extracted. handle appropriately or do nothing
			console.log(await response.text());
		}

		return response;
	});
}

export function deleteExtractedTextsForDocument(documentId) {
	return deleteRequest(`/extracted_texts/documents/${documentId}`);
}

export function deleteIdeaRefsForDocument(documentId) {
	return deleteRequest(`/idea_refs/documents/${documentId}`);
}
