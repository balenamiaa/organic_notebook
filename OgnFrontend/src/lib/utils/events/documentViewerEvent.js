export function documentViewerEvent(node) {
	let lastSelectionTimer;
	let posInScreen;
	function getPositionInScreen(event) {
		return {
			x: event.clientX + window.scrollX,
			y: event.clientY + window.scrollY,
		};
	}
	function onSelection() {
		const _posInScreen = posInScreen;
		clearTimeout(lastSelectionTimer);
		lastSelectionTimer = setTimeout(() => {
			const selection = document.getSelection();
			const selectionText = selection.toString();
			if (
				node.contains(selection.anchorNode) &&
				node.contains(selection.focusNode) &&
				!selection.isCollapsed &&
				selectionText.trim() !== ''
			) {
				node.dispatchEvent(
					new CustomEvent('selectionEnd', {
						detail: {
							posInScreen: _posInScreen,
							selectionText,
							focusNode: selection.focusNode,
						},
					}),
				);
			}
		}, 500);
	}
	function onMouseMove(event) {
		posInScreen = getPositionInScreen(event);
	}

	node.addEventListener('mousemove', onMouseMove);
	document.addEventListener('selectionchange', onSelection);
	return {
		destroy() {
			node.removeEventListener('mousemove', onMouseMove);
			document.removeEventListener('selectionchange', onSelection);
		},
	};
}
