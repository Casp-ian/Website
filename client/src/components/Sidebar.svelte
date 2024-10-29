<script>
	import Footer from '$components/Footer.svelte';
	import Header from '$components/Header.svelte';
</script>

<div id="wrapper">
	<Header />

	<Footer />
</div>

<style>
	#wrapper {
		display: flex;
		flex-direction: column;

		height: 100%;
		justify-content: space-between;
		align-content: center;
	}
</style>
