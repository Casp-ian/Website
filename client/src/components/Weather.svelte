<script lang="ts">
	import { onMount } from 'svelte';
	import { weather } from '$stores/weather';

	let current_weather = 'none';

	// weathers: rain, snow, sun, night
	// export let weather = 'night';

	let generatorDelay = 400;
	let maxParticles = 20;

	let precipitants: { character: string; x: number; y: number }[] = [];

	let width = 0;
	let height = 0;

	onMount(async () => {
		// TODO maybe just get rid of the interval when weather == 'none', dont think it has too much impact tho
		// tho it could be neat to have a specific `spawnSnow()` and `moveSnow()` and so forth
		setInterval(() => {
			spawnPrecipitation();
			movePrecipitation();
			clearPrecipitation();
		}, generatorDelay);
	});

	weather.subscribe((value) => {
		precipitants = [];
		current_weather = value;
	});

	function spawnPrecipitation() {
		if (current_weather === 'none') {
			return;
		}

		let character = {
			character: '+',

			// todo make these adjustments more exact
			x: Math.floor(Math.random() * (width - 20)), // adjust for character width
			y: Math.floor(Math.random() * (height - 40)), // adjust for character height
		};

		precipitants.push(character);

		precipitants = precipitants;
	}

	function movePrecipitation() {
		if (current_weather === 'none') {
			return;
		}

		precipitants.forEach((element) => {
			if (element.character === '+') {
				element.character = 'x';
			} else if (element.character === 'x') {
				element.character = '+';
			}
		});
	}

	function clearPrecipitation() {
		if (current_weather === 'none') {
			return;
		}

		if (precipitants.length > maxParticles) {
			precipitants.shift();
		}
	}
</script>

<svelte:window bind:innerWidth={width} bind:innerHeight={height} />

{#each precipitants as precipitant}
	<p class="particle" style="left: {precipitant.x}px; top: {precipitant.y}px;">
		{precipitant.character}
	</p>
{/each}

<!--
<p>{current_weather}</p>
-->

<style>
	.particle {
		position: absolute;
		user-select: none;
		pointer-events: none;
	}
</style>
