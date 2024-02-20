<script>
    import WelcomeScreen from "../components/WelcomeScreen.svelte";

    export let params;
</script>


<!--welcome screen or something?-->
<WelcomeScreen
    backgroundImageUrl="/src/assets/artifact.png"
    headerHeight="4rem"
>
    <h1>
        WELCOME
    </h1>
</WelcomeScreen>

<div id="text">
    <h1>HAHAHA HELLOOOOOOOOOOO</h1>

    <p>
        This is my portfolio website, it is still very much in progress, but work is being done. <br>
        I am planning to show of my fun projects here and hopefully market myself as a good software engineer.
    </p>
</div>


<style>
    #text {
        margin: 10rem;
    }
</style>