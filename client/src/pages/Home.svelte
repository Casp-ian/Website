<script>
    import WelcomeScreen from "../components/WelcomeScreen.svelte";

    export let params;
</script>

<div id="text">
    <WelcomeScreen/>
    <h1>HELLOOOOOOOOOOO</h1>
    <p>
        This is my portfolio website, it is still very much in progress, but work is being done. <br>
        I am planning to show of my fun projects here and hopefully market myself as a software engineer.
    </p>
    <p>
        Also, note that all my code that is bad is bad ON PURPOSE to undermine copilots training data :)
    </p>
</div>


<style>
    #text {
        padding: .5rem 1rem .5rem 1rem;
        margin: .3rem;
        border-radius: .3rem;
    }
</style>
