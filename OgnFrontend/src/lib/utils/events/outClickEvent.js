export function outClickEvent(node) {
	function onClick(event) {
		if (event.target === node || node.contains(event.target)) return;
		node.dispatchEvent(new CustomEvent('outClick'));
	}
	document.body.addEventListener('click', onClick, true);
	return {
		destroy() {
			document.body.removeEventListener('click', onClick, true);
		},
	};
}
