<script>
    import WelcomeScreen from "../components/WelcomeScreen.svelte";
    import Footer from "../components/Footer.svelte";

    export let params;
</script>

<!--welcome screen or something?-->
<WelcomeScreen>
    <h1>
        WELCOME
    </h1>
</WelcomeScreen>

<div id="text">
    <h1>HELLOOOOOOOOOOO</h1>

    <p>
        This is my portfolio website, it is still very much in progress, but work is being done. <br>
        I am planning to show of my fun projects here and hopefully market myself as a software engineer.
    </p>
</div>

<Footer></Footer>


<style>
    #text {
        padding: 1rem 2rem 1rem 2rem;
        margin: .3rem;
        border-radius: .3rem;
        background-color: var(--back-color);
    }
</style>
