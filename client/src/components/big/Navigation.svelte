<script lang="ts">
    import {page} from '$app/state';
    import type {Route} from "$lib/navigation";
    import {pages} from "$lib/navigation";

    function isActive(path: string): boolean {
        return page.url.pathname.includes(path);
    }
</script>

<nav id="wrapper">
    <div class="column">
        {#each pages as page}
            <div class="row">
                <a href="{page.path}" class:active="{isActive(page.path)}">
                    <img src="{page.icon}" alt="">
                    {page.name}
                </a>
                {#if isActive(page.path)}
                    {@render children(page)}
                {/if}
            </div>
        {/each}
    </div>
</nav>

{#snippet children(parent: Route)}
    <div class="column">
        {#each parent.children as page}
            <a href="{page.path}" class:active="{isActive(page.path)}">
                <img src="{page.icon}" alt="">
                {page.name}
            </a>
        {/each}
    </div>
{/snippet}

<style>

    img {
        height: 20px;
        width: auto;
        vertical-align: middle;
    }

    a {
        width: 9vw;
        color: white;
        text-decoration: none;
        text-align: left;

        text-wrap: nowrap;

        border-top-left-radius: 7px;
        border-bottom-left-radius: 7px;

        background-image: linear-gradient(to right, rgb(100, 100, 100) 50%, white);

        margin: 2px 0.5vw 2px 0.5vw;

        overflow: hidden;
    }

    .active {
        background-image: linear-gradient(to right, rgba(100, 100, 100, 0.8) 50%, white),
        linear-gradient(135deg, blueviolet 10%, rgba(0, 0, 0, 0));
    }

    nav {
        display: flex;
        flex-direction: row;
    }

    .column {
        height: max-content;
        display: flex;
        flex-direction: column;
    }

    .row {
        height: 25px;
        display: flex;
        flex-direction: row;
    }
</style>
