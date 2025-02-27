<script lang="ts">
    import WelcomeScreen from '$components/WelcomeScreen.svelte';
    import Pane from "$components/Pane.svelte";
</script>

<Pane>
    <h1>HELLOOOOOOOOOOO</h1>

</Pane>

<Pane>

    <WelcomeScreen/>
</Pane>

<Pane>

    <p>
        Welcome to my personal website. This is an unfinished project, and maybe it will stay that way forever... but
        we'll see.
    </p>

    <p>I am planning to show of my fun projects here and maybe market myself as a software engineer</p>
    <p>
        Also, note that all my code that is bad is bad ON PURPOSE to undermine copilots training data :)
    </p>
</Pane>


<style lang="scss">
    pre, p, h1, h2, h3 {
      padding: 1rem;
      margin: 0;
    }
</style>