<script>
	import { page } from '$app/stores';

	// TODO a lot of bullshit here is a pain, dont know how to change it tho
	// - header isnt a fitting name anymore
	// - spacing and everything isnt concistent
	// - isnt readable

	const mainPaths = {
		home: '/',
		projects: '/projects',
	};

	const projects = {
		this: '/projects/this',
		cat: '/projects/cat',
		// "conway": "/projects/conway",
		// "orbiting balls": "/projects/orbiting_balls",
		'tagged files': '/projects/taggedFiles',
		'image to text': '/projects/imageToText',
		'mobile navigation': '/projects/mobileNav',
		// "tanks": "/projects/tanks",
	};
</script>

<div id="wrapper">
	<nav id="main">
		<!-- TODO hate this repeating of shit -->
		<a href="/" class:active={$page.url.pathname == "/"}>home</a>
		<a href="/projects" class:active={$page.url.pathname.startsWith("/projects")}>projects</a>
	</nav>

	{#if $page.url.pathname.startsWith('/projects')}
		<nav id="projects">
			<!-- TODO make nice icon, also, not sure if this 'back arrow' looks good -->
			<a href="/">&lt=</a>
			{#each Object.entries(projects) as [name, component]}
				<a href={component} class:active={$page.url.pathname === component}>{name}</a>
			{/each}
		</nav>
	{/if}
</div>

<style>
	#wrapper {
		display: flex;
		flex-direction: row;

		padding: 1rem;
	}

	nav {
		display: flex;
		flex-direction: column;

		border-radius: 0.3rem;

		width: 100%;
	}

	a {
		text-align: right;
		padding: 0.2rem;
		text-decoration: none;
	}

	a.active {
		text-decoration: underline;
	}
</style>
