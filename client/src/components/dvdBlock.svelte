<script lang="ts">
	import { onDestroy, onMount } from 'svelte';

	export let text = 'Hello :3';

	let block: HTMLElement;
	let area: HTMLElement;

	let test = { x: 2500, y: 2500, color: 'pink' };

	let up = true;
	let left = true;

	// TODO randomise these >:)
	// TODO these are relative to area width and height, i might want to fix that
	const speedUp = 50;
	const speedLeft = 27;

	const max = 5000;

	let interval: number;
	onMount(async () => {
		adjust();

		tick();
		interval = setInterval(tick, 50);
	});

	onDestroy(async () => {
		clearInterval(interval);
	});

	function adjust() {
		area.style.width = 'calc( 100% - ' + block.clientWidth + 'px)';
		area.style.height = 'calc( 100% - ' + block.clientHeight + 'px)';
	}

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
			test.color = 'pink';
		}

		block.style.color = test.color;
		block.style.left = 'calc((' + test.x + ' / ' + max + ') * 100%)';
		block.style.top = 'calc((' + test.y + ' / ' + max + ') * 100%)';
	}
</script>

<div id="wrapper">
	<div id="area" bind:this={area}>
		<p id="block" bind:this={block}>
			{text}
		</p>
	</div>
</div>

<style>
	#wrapper {
		width: calc(100% - 2px);
		height: calc(100% - 2px);
		border: solid 1px black;
	}

	#area {
		/* width and height gets adjusted from js */
	}

	#block {
		width: fit-content;
		height: fit-content;
		position: relative;

		font-size: 40px;
		line-height: 40px;
		margin: 0;
		padding: 0;
	}
</style>
