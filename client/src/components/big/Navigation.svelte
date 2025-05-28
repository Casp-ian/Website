<script lang="ts">
    import {page} from '$app/state';
    import type {Route} from "$lib/navigation";
    import {pages} from "$lib/navigation";

    function isActive(path: string): boolean {
        return page.url.pathname.includes(path);
    }
</script>


<nav>
    {#each pages as page}
        {#if page.children.length === 0}
            <div class="column">
                <a href="{page.path}" class:active="{isActive(page.path)}">
                    <img src="{page.icon}" alt="">
                    {page.name}
                </a>
            </div>
        {:else}
            <div class="test">
                <a href="{page.path}" class:active="{isActive(page.path)}">
                    <img src="{page.icon}" alt="">
                    {page.name}
                </a>
                <!--{#if isActive(page.path)}-->
                {@render children(page)}
                <!--{/if}-->
            </div>
        {/if}

    {/each}
</nav>

{#snippet children(parent: Route)}
    <div class="offset column">
        {#each parent.children as page}
            <a href="{page.path}" class:active="{isActive(page.path)}">
                <img src="{page.icon}" alt="">
                {page.name}
            </a>
        {/each}
    </div>
{/snippet}


<style>
    nav {
        margin: 1rem;
        display: flex;
        flex-direction: column;
    }

    img {
        height: 20px;
        width: auto;
        vertical-align: middle;
        filter: invert(1);
    }

    a {
        color: white;
        text-decoration: none;
        text-align: left;

        text-wrap: nowrap;

        border-radius: 1rem;
        backdrop-filter: blur(5px) brightness(50%);

        /*background-image: linear-gradient(to right, rgb(100, 100, 100) 50%, white);*/

        margin: 0.25rem;
        padding: 0.1rem 0.2rem;
    }

    .active {
        text-decoration: underline;
    }

    .column {
        height: max-content;
        display: flex;
        flex-direction: column;
        flex-grow: 1;
    }

    .test {
        border-top-left-radius: 1rem;
        border-bottom-left-radius: 1rem;

        display: flex;
        flex-direction: row;
    }

    .test > a {
        writing-mode: vertical-lr;

        transform-origin: center;
        transform: rotate(0.5turn);

        padding: 0.2rem 0.1rem;

        /*margin: auto 5px;*/
        /*background: red;*/
        /*height: 2rem;*/
    }
</style>
