// TODO somehow run this code earlier, so mobile users dont get flashed with the desktop version

// TODO rename to isnt horizontal or something
export function isVertical() {
	return window.innerHeight / window.innerWidth > 1.0;
}

export function isTouch() {
	return window.matchMedia('(pointer: coarse)').matches;
}
