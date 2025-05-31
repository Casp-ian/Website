<script lang="ts">
    import {flip} from "svelte/animate";
    import {fly} from "svelte/transition";
    import {fade} from "svelte/transition";
    import {onMount} from "svelte";


	// TODO
    let notifications: string[] = ["A"];

    onMount(() => {
        // notifications.push("A");
        // setInterval(() => {
        //     // console.log("a");
        //     notifications = notifications;
        //     notifications.push("A" + Math.random());
        //     // notifications.push("fuck you!");
        // }, 500);
        // setInterval(() => {
        //     // console.log("a");
        //     notifications = notifications.slice(1, notifications.length);
        //     // notifications.push("fuck you!");
        // }, 1600);
    })
</script>

<div id="wrapper">
    {#each notifications as item, index (item)}
        <!--TODO animations, may need custom function transition-->
        <div animate:flip id="notification">
            <p>{item}</p>
        </div>
    {/each}
</div>


<style>
    #wrapper {
        margin: 1rem;
    }

    #notification {
        background: black;
    }
</style>