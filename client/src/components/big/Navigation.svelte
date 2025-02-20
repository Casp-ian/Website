<script lang="ts">
    import {pages, subPages} from "$lib/navigation";
</script>

<nav id="wrapper">
    <div class="column">
        {#each pages as page}
            <a href="{page.path}"><img src="{page.icon}" alt="">{page.name}</a>
        {/each}
    </div>
    <div class="column">
        {#each subPages as subPage}
            <a href="{subPage.path}"><img src="{subPage.icon}" alt="">{subPage.name}</a>
        {/each}
    </div>
</nav>

<style>

    img {
        height: 20px;
        width: auto;
        vertical-align: middle;
    }

    nav {
        display: flex;
        flex-direction: row;

        padding: 1rem;

        border-radius: 0.3rem;

        width: 100%;
    }

    .column {
        display: flex;
        flex-direction: column;

        text-align: left;
        padding: 0.2rem;
        text-decoration: none;
    }

</style>
