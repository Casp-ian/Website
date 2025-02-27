<script lang="ts">
    import {page} from '$app/state';
    import Pane from "$components/Pane.svelte";
</script>

<Pane>
    <h1>oopsie :3</h1>
    <p>got {page.status}'ed</p>
    <p>{page.error?.message ?? ''}</p>
</Pane>

<style>
    pre, p, h1, h2, h3 {
        padding: 1rem;
        text-align: center;

        margin: 0;
    }
</style>
