export const baseUrl = 'http://127.0.0.1:8080'
export const apiUrl = `${baseUrl}/api`

export const contentType = {
	json: { 'content-type': 'application/json' }
}

export function fetchFromBase(url, init) {
	return fetch(`${baseUrl}${url}`, { ...init })
}

export function fetchApi(url, init) {
	return fetch(`${apiUrl}${url}`, { ...init })
}

export function getRequest(url, init) {
	return fetchApi(url, { method: 'GET', ...init })
}

export function postRequest(url, init) {
	return fetchApi(url, { method: 'POST', ...init })
}

export function deleteRequest(url, init) {
	return fetchApi(url, { method: 'DELETE', ...init })
}