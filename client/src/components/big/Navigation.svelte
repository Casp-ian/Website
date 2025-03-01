<script lang="ts">
    import {page} from '$app/state';
    import type {Route} from "$lib/navigation";
    import {pages} from "$lib/navigation";

    function isActive(path: string): boolean {
        return page.url.pathname.includes(path);
    }
</script>


<nav class="column">
    {#each pages as page}
        <div class="column">
            <a href="{page.path}" class:active="{isActive(page.path)}">
                <img src="{page.icon}" alt="">
                {page.name}
            </a>
            <!--{#if isActive(page.path)}-->
            {@render children(page)}
            <!--{/if}-->
        </div>
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
        position: fixed;
        /*top: 1rem;*/
        bottom: 5rem;
        left: 1vw;
        pointer-events: auto;
    }

    /* if not mobile we but the nav at the top */
    @media (min-width: 500px) {
        nav {
            top: 1rem;
        }
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

        /*border-top-left-radius: 7px;*/
        /*border-bottom-left-radius: 7px;*/

        /*background-image: linear-gradient(to right, rgb(100, 100, 100) 50%, white);*/

        margin: 0.1rem 0;

        overflow: hidden;
    }

    .active {
        text-decoration: underline;
        /*background-image: linear-gradient(to right, rgba(100, 100, 100, 0.8) 50%, white),*/
        /*linear-gradient(135deg, blueviolet 10%, rgba(0, 0, 0, 0));*/
    }

    nav {
        display: flex;
        flex-direction: row;
    }

    .column {
        width: 10vw;
        height: max-content;
        display: flex;
        flex-direction: column;
    }

    .offset {
        margin-left: 4vw;
    }
</style>
