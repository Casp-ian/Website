// TODO somehow run this code earlier, so mobile users dont get flashed with the desktop version
export function isVertical() {
	return window.innerHeight / window.innerWidth > 1.2;
}

export function isTouch() {
	return window.matchMedia('(pointer: coarse)').matches;
}
