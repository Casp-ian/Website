<script>
	import hljs from 'highlight.js/lib/core';
	import javascript from 'highlight.js/lib/languages/javascript';
	import rust from 'highlight.js/lib/languages/rust';
	import xml from 'highlight.js/lib/languages/xml';

	hljs.registerLanguage('javascript', javascript);
	hljs.registerLanguage('rust', rust);
	hljs.registerLanguage('xml', xml);

	import { onMount } from 'svelte';

	onMount(async () => {
		codeElement.innerHTML = hljs.highlight(text, { language: language }).value;
		// hljs.hightlightAll();
	});

	export let text;
	export let language;

	let codeElement;

	let copyText = '⎘';

	function copy() {
		navigator.clipboard.writeText(text);
		copyText = '✓';
		setTimeout(() => {
			copyText = '⎘';
		}, 1000);
	}
</script>

<div id="wrapper">
	<div id="copy" on:click={copy}>
		{copyText}
	</div>
	<pre bind:this={codeElement} id="code" />
</div>

<style>
	@import url('https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/a11y-dark.min.css');
	#wrapper {
		background-color: black;
		width: 100%;
		flex-direction: row;
	}

	#copy {
		color: white;
		float: right;
		width: 1rem;
		height: 1rem;
	}

	#code {
		width: 100%;
		padding: 1rem;
		color: white;
	}
</style>
