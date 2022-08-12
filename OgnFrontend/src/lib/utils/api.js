import { assert } from './utils';

export const baseUrl = 'http://127.0.0.1:80';
export const apiUrl = `${baseUrl}/api`;

export const contentType = {
	json: { 'Content-Type': 'application/json' },
};

export function fetchFromBase(url, init) {
	return fetch(`${baseUrl}${url}`, { ...init });
}

export function fetchApi(url, init) {
	return fetch(`${apiUrl}${url}`, { ...init });
}

export function getRequest(url, init) {
	return fetchApi(url, { method: 'GET', ...init });
}

export function postRequest(url, init) {
	return fetchApi(url, { method: 'POST', ...init });
}

export function deleteRequest(url, init) {
	return fetchApi(url, { method: 'DELETE', ...init });
}

export function getPaginatedRequest(url, options, init) {
	if (options['pageNum']) {
		assert(
			options.hasOwnProperty('pageSize'),
			'either specify both pageNum and pageSize or neither',
		);
		assert(url.charAt(url.length - 1) != '/', 'url expecting pagination should not end with /');

		let pageNum = options['pageNum'];
		let pageSize = options['pageSize'];
		return getRequest(`${url}?page_num=${pageNum}&page_size=${pageSize}`, init);
	} else {
		assert(
			!options.hasOwnProperty('pageNum'),
			'either specify both pageNum and pageSize or neither',
		);

		return getRequest(url, init);
	}
}

export function asJson(requestPromise) {
	return requestPromise.then(async (response) => {
		let json = await response.json();

		// NOTE: handle more status codes here when the api does not only use 200 to designate success.
		if (response.status == 200) {
			return json;
		} else {
			return json.then((err) => {
				throw err;
			});
		}
	});
}
