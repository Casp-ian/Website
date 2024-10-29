<script lang="ts">
	import { page } from '$app/stores';
</script>

<div id="wrapper">
	<h1>oopsie :3</h1>
	<p>got {$page.status}'ed</p>
	<p>{$page.error?.message ?? ''}</p>
</div>

<style>
	#wrapper {
		position: relative;
		width: 100%;
		top: 50%;
		transform: translateY(-50%);
		text-align: center;
	}
</style>
