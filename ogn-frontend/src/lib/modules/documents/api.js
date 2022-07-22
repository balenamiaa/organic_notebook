import {getRequest, postRequest} from '$lib/utils/api.js'

export function getDocuments() {
    return getRequest('/documents')
}

export function getDocumentEntry(id) {
    return getRequest(`/documents/${id}`)
}

export function uploadDocument(files) {
    const formData = new FormData()
    for (let i = 0; i < files.length; i++) {
        formData.append(files[i].name, files[i])
    }
    return postRequest(`/upload_document`, {body: formData})
}
