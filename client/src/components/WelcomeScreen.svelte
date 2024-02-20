<script>
    import { fly } from 'svelte/transition';

    // css will handle fine if backgroundImageUrl is empty
    export let backgroundImageUrl;
    backgroundImageUrl = 'url(' + backgroundImageUrl + ')'

    export let headerHeight = 0;

    let scroll;
</script>


<!--<div id="welcome" style="&#45;&#45;background-image: {backgroundImageUrl}; &#45;&#45;headerHeight: {headerHeight}">-->
<!--    <slot></slot>-->
<!--</div>-->

<div id="temp" style="--headerHeight: {headerHeight}">
    <h3>⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
        ⠀⠀⠀⠀⠀⠀⢕⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⠀⣫⣿⣿⠀⠀⠀⠀⠀⠀⠀⡈⣿⣿⣿⣿⣿⠀⠀⠀⡈⣫⣿⣿⣿⣿⢕⠀⠀⠀⠀⢕⣿⣿⠀⠀⠀⠀⠀⠀⣿⣿⣫⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀    <br>
        ⠀⠀⠀⠀⣿⣿⣿⡈⠀⠀⠀⣫⠀⠀⠀⠀⠀⢕⣿⣿⣿⣿⠀⠀⠀⠀⠀⣿⣿⣿⠀⠀⠀⢕⠀⠀⠀⢕⣿⣿⠀⠀⡈⣿⣿⣿⠀⠀⢕⣿⣿⠀⠀⠀⠀⠀⣿⣿⣿⣿⣫⠀⠀⠀⠀⠀⣿⣿⣿⣿⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀    <br>
        ⠀⠀⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⣿⣿⠀⣫⣿⣫⠀⠀⠀⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⢕⣿⣿⠀⠀⠀⠀⣿⣿⡈⠀⢕⣿⣿⠀⠀⠀⠀⣫⣿⣫⠀⣿⣿⡈⠀⠀⠀⠀⣿⣿⡈⣿⣿⡈⠀⠀⣿⣿⠀⠀⠀⠀    <br>
        ⠀⠀⠀⣿⣿⡈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⣿⣿⢕⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⠀⠀⠀⢕⣿⣿⣿⣿⣿⣿⣿⢕⠀⠀⢕⣿⣿⠀⠀⠀⡈⣿⣿⠀⠀⠀⣿⣿⠀⠀⠀⠀⣿⣿⠀⠀⣫⣿⣿⠀⣿⣿⠀⠀⠀⠀    <br>
        ⠀⠀⠀⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⢕⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⢕⣿⣿⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⣿⣿⠀⠀⠀⠀⣿⣿⣿⣿⠀⠀⠀⠀    <br>
        ⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⡈⠀⣿⣿⣿⠀⠀⠀⠀⠀⢕⣿⣿⠀⠀⣿⣿⣫⣫⣿⣿⣿⣿⠀⠀⢕⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⢕⣿⣿⠀⠀⣿⣿⢕⠀⠀⠀⠀⠀⣿⣿⣿⠀⠀⣿⣿⠀⠀⠀⠀⠀⣿⣿⣿⠀⠀⠀⠀    <br>
        ⠀⠀⠀⠀⠀⠀⠀⡈⢕⢕⡈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡈⢕⢕⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀    <br>
        ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀    <br>
    </h3>
</div>

<svelte:window bind:scrollY={scroll} />

{#if scroll < 15}
    <!--a pointer downwards so the user knows that there is more to see if you scroll-->
    <div transition:fly={{ y: 100 }} id="pointer">
        <p>scroll down</p>
    </div>
{/if}


<style>
    #welcome {
        height: calc(100vh - var(--headerHeight));
        justify-content: center;
        align-items: center;
        display: flex;
        flex-direction: column;

        background-color: antiquewhite;

        background: var(--background-image) no-repeat center;
        background-size: cover;

        text-shadow: 0 0 3px white, 0 0 5px white;
    }

    #temp {
        height: calc(100vh - var(--headerHeight));
        justify-content: center;
        align-items: center;
        display: flex;
        flex-direction: column;


        background: rgb(27, 18, 18);
        text-shadow: 0 0 3px white, 0 0 5px white;
    }

    #pointer {
        position: absolute;
        top: 95%;
        left: 50%;
        transform: translate(-50%, -50%);

        background: orange;
        border-radius: 25px;

        font-size: 1.5rem;

    }

    #pointer>p {
        margin: 1rem;
    }
</style>