import { deleteRequest, fetchFromBase, getRequest, postRequest } from '$lib/utils/api.js'

export function getDocuments(page = 0, pageSize = 10) {
	return getRequest(`/documents?page_num=${page}&page_size=${pageSize}`)
}

export function getDocumentEntry(id) {
	return getRequest(`/documents/${id}`)
}

export function uploadDocument(files) {
	const formData = new FormData()
	for (let i = 0; i < files.length; i++) {
		formData.append(files[i].name, files[i])
	}
	return postRequest('/documents', { body: formData })
}

export function getDocumentFile(document) {
	return fetchFromBase(`/host/static/${document.id}.${document.filetype}`)
}

export function deleteDocument(documentId) {
	return deleteRequest(`/documents/${documentId}`)
}

export function deleteDocumentExtractedText(documentId) {
	return deleteRequest(`/extracted_texts/${documentId}`)
}