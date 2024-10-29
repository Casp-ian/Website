export function isVertical() {
  return window.innerHeight / window.innerWidth > 1.2;
}

export function isTouch() {
  return window.matchMedia("(pointer: coarse)").matches;
}
