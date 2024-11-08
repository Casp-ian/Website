<script lang="ts">
	export let src;
	export let alt;

	let invisible = true;

	function close() {
		invisible = true;
	}

	function open() {
		invisible = false;
	}

	function onKeyDown(e: Event) {
		if (e.keyCode == 27) {
			close();
		}
	}
</script>

<!-- the normal image, as if pretty image was never even used -->
<img on:click={open} {src} {alt} />

<!-- big image, hidden untill image is clicked -->
<div class="big" on:click={close} class:invisible>
	<img class="big" {src} {alt} />
</div>

<svelte:window on:keydown={onKeyDown} />

<style>
	img {
		width: 100%;
	}

	img.big {
		position: fixed;
		top: 50%;
		left: 50%;

		transform: translate(-50%, -50%);

		max-width: 95vw;
		max-height: 95vh;

		width: auto;
		height: auto;
	}

	div.big {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(150, 150, 150, 0.4);
	}

	div.invisible {
		visibility: hidden;
	}
</style>
