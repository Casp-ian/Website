<script lang="ts">
	import { page } from '$app/state';
</script>

<div id="wrapper">
	<h1>oopsie :3</h1>
	<p>got {page.status}'ed</p>
	<p>{page.error?.message ?? ''}</p>
</div>

<style>
	#wrapper {
		text-align: center;
	}
</style>
