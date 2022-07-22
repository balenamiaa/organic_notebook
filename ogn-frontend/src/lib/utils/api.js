export const apiUrl = 'http://localhost:8080/api'

export function fetchApi(url, init) {
    return fetch(`${apiUrl}${url}`, {...init})
}

export function getRequest(url, init) {
    return fetchApi(url, {method: 'GET', ...init})
}

export function postRequest(url, init) {
    return fetchApi(url, {method: 'POST', ...init})
}
