export function groupBy(xs, key = 'id') {
	return xs.reduce(function (rv, x) {
		if (!rv[x[key]]) {
			rv[x[key]] = [];
		}
		rv[x[key]].push(x);
		return rv;
	}, {});
}

export function groupBySingle(xs, key = 'id') {
	return xs.reduce(function (rv, x) {
		rv[x[key]] = x;
		return rv;
	}, {});
}