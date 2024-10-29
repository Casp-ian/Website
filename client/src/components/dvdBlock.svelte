<script>
	import { onMount } from 'svelte';

	let test = { x: 0, y: 0, color: 'red' };

	let up = true;
	let left = true;

	// TODO randomise these >:)
	const speedUp = 50;
	const speedLeft = 27;

	const max = 5000;

	onMount(async () => {
		setInterval(tick, 50);
	});

	function tick() {
		if (up) {
			test.y += speedUp;
		} else {
			test.y -= speedUp;
		}

		if (left) {
			test.x += speedLeft;
		} else {
			test.x -= speedLeft;
		}

		if (test.y >= max) {
			up = false;
			test.color = 'red';
		} else if (test.y <= 0) {
			up = true;
			test.color = 'green';
		}

		if (test.x >= max) {
			left = false;
			test.color = 'blue';
		} else if (test.x <= 0) {
			left = true;
			test.color = 'yellow';
		}

		test = test;
	}
</script>

<p
	id="block"
	style="top: calc({test.y / max} * 100%); left: calc({test.x / max} * 100%); color: {test.color}"
>
	HELLO!
</p>

<style>
	#block {
		width: fit-content;
		height: fit-content;
		position: relative;

		transform: translate(-50%, -50%);

		font-size: 40px;
		line-height: 40px;
	}
</style>
