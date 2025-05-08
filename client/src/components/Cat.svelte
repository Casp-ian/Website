<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	export let startX = 0;
	export let startY = 0;

	enum State {
		Stunned,
		Idle,
		Running,
		Purring,
		Sleeping,
	}
	enum Direction {
		Up,
		Right,
		Down,
		Left,
		TopRight,
		BottomRight,
		BottomLeft,
		TopLeft,
	}

	let catState: State = State.Stunned;
	let runDirection: Direction = Direction.Up;
	// this is in frames
	let catStateTimer = 9;

	// used for animation mainly
	let imageOffset = { x: -32, y: -96 };
	let even = false;

	// used for movement mainly
	let mousePos = { x: startX, y: startY };
	let catPos = { x: startX, y: startY };

	let distance = 0;
	let catVector = { x: 0, y: 0 };

	function manageCatState() {
		// console.log("cat is: " + catState + " and is waiting for " + catStateTimer);
		if ( catStateTimer > 0 ) {
			catStateTimer -= 1;
			return;
		}

		switch (catState) {
			case State.Stunned:
				if (distance > 50) {
					catState = State.Running;
				} else {
					catState = State.Idle;
				}
				break;
			case State.Idle:
				if (distance > 40) {
					catState = State.Stunned;
					catStateTimer = 9;
					resetPet(); // every other exit out of idle state needs reset pet
				} else if (checkIfPet()) {
					catState = State.Purring;
					catStateTimer = 9;
				}
				// TODO do idle animation maybe
				// TODO fall asleep maybe
				break;
			case State.Running:
				if (distance < 20) {
					catState = State.Idle;
				} else {
					run();
				}
				break;
			case State.Purring:
				catState = State.Idle;
				break;
      case State.Sleeping:
				catState = State.Idle;
				break;
		}

		setRunDirection();
	}

	function run() {
		let speed = even ? 13 : 12; //differ speed to make movement more "natural"
		catPos.x += catVector.x * speed;
		catPos.y += catVector.y * speed;

		// clamp x
		if (catPos.x > ( window.innerWidth - 16 )) {
			catPos.x = ( window.innerWidth - 16 );
		} else if (catPos.x < 16) {
			catPos.x = 16;
		}

		// clamp y
		if (catPos.y > ( window.innerHeight - 16 )) {
			catPos.y = ( window.innerHeight - 16 );
		} else if (catPos.y < 16) {
			catPos.y = 16;
		}

		catPos = catPos;
	}
	
	function update() {
		// cat movement logic
		let diff = { x: mousePos.x - catPos.x, y: mousePos.y - catPos.y + window.scrollY };
		distance = Math.sqrt(diff.x * diff.x + diff.y * diff.y);
		catVector = { x: diff.x / distance, y: diff.y / distance };

		manageCatState();
	}

	function animate() {
		even = !even;

		let y = even ? 0 : -32;

		// stunning
		if (catState == State.Stunned) {
			imageOffset = { x: -288, y };
		}

		// running
		if (catState == State.Running) {
			switch (runDirection) {
				case 0:
					imageOffset = { x: -32, y };
					break;
				case 1:
					imageOffset = { x: -64, y };
					break;
				case 2:
					imageOffset = { x: -96, y };
					break;
				case 3:
					imageOffset = { x: -128, y };
					break;
				case 4:
					imageOffset = { x: -160, y };
					break;
				case 5:
					imageOffset = { x: -192, y };
					break;
				case 6:
					imageOffset = { x: -224, y };
					break;
				case 7:
					imageOffset = { x: -256, y };
					break;
			}
		}

		// sitting
		if (catState == State.Idle) {
			imageOffset = { x: 0, y: -32 };
		}

		// petting
		if (catState == State.Purring) {
			imageOffset = { x: -352, y };
		}
	}

	function setRunDirection() {
		// cardinal directions
		if (catVector.y < -0.88) {
			runDirection = Direction.Up;
			return;
		}
		if (catVector.x > 0.88) {
			runDirection = Direction.Right;
			return;
		}
		if (catVector.y > 0.88) {
			runDirection = Direction.Down;
			return;
		}
		if (catVector.x < -0.88) {
			runDirection = Direction.Left;
			return;
		}

		// diagonals
		if (catVector.x > 0 && catVector.y < 0) {
			runDirection = Direction.TopRight;
		}
		if (catVector.x > 0 && catVector.y > 0) {
			runDirection = Direction.BottomRight;
		}
		if (catVector.x < 0 && catVector.y > 0) {
			runDirection = Direction.BottomLeft;
		}
		if (catVector.x < 0 && catVector.y < 0) {
			runDirection = Direction.TopLeft;
		}
	}

	let petScore = 0;
	// this only uses cardinal directions
	let lastPetDirection = Direction.Up;
	
	function checkIfPet() {
		let newPetQuarter;
		if (catVector.y > 0) {
			if (catVector.x > 0) {
				newPetQuarter = Direction.TopRight;
			} else {
				newPetQuarter = Direction.TopLeft;
			}
		} else {
			if (catVector.x > 0) {
				newPetQuarter = Direction.BottomRight;
			} else {
				newPetQuarter = Direction.BottomLeft;
			}
		}

		if (newPetQuarter != lastPetDirection) {
			petScore++;
			lastPetDirection = newPetQuarter;
		}

		if (petScore >= 4) {
			// TODO: This does not always work because the cat is not always above the body element
			document.body.style.cursor = "grab";
		}
		
		if (petScore >= 9) {
			resetPet();
			console.log('meow');
			return true;
		}
		return false;
	}

	function resetPet() {
		petScore = 0;
		document.body.style.cursor = "unset";
	}

	function updateMouse(event: MouseEvent) {
		mousePos.x = event.clientX;
		mousePos.y = event.clientY;
	}

	let updateInterval: number;
	let animateInterval: number;

	onMount(async () => {
		updateInterval = setInterval(update, 110);
		animateInterval = setInterval(animate, 110);
		document.addEventListener('mousemove', updateMouse);
	});

	onDestroy(async () => {
		clearInterval(updateInterval);
		clearInterval(animateInterval);
	});
</script>

<div
	id="cat"
	style="top: {catPos.y}px; left: {catPos.x}px; background-position-x: {imageOffset.x}px; background-position-y: {imageOffset.y}px;"
>
</div>

<style>
	#cat {
		z-index: 1;
		pointer-events: none;
		position: fixed;
		width: 32px;
		height: 32px;
		background-image: url('$assets/oneko2.png');
		image-rendering: pixelated;
		transform: translate(-50%, -50%);
	}
</style>
