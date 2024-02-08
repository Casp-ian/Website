<script>
    import BlogItem from "../components/BlogItem.svelte";
    import format from "../lib/format.js";

    export let params;

    export let items = [
        {
            id: "1",
            date: 1706773192213,
            text: "Helloooo!",
        },
        {
            id: "2",
            date: 1706472180713,
            text: "Helloooouuuuuwdjomasmd!",
        },
    ];

    const now = Date.now();

    function processItems() {
        items = items.sort((a, b) => b.date - a.date);

        let previousDate = now;
        for (let i = 0; i < items.length; i++) {
            const thisDate = items[i].date;
            items[i].diff = (previousDate - thisDate) / 500000;
            previousDate = thisDate;
        }
    }
    processItems();

    let blogItem = params.id;
    // TODO scroll to blogitem


    let test = new Date(now);
    console.log(test);
</script>


<div id="timeline">
    <a class="date" href={"/blog"} on:click={() => {blogItem = undefined}} style={"margin-top: 100px;"}>
        {format.dateShort(now)}
    </a>
    {#each items as item}
        <div class="blogItem" style={"margin-top: " + item.diff + "px;"}>
            <a class="date" href={"/blog/" + item.id} on:click={() => {blogItem = item.id}}>
                {format.dateShort(item.date)}
            </a>
            <a class="text" href={"/blog/" + item.id} on:click={() => {blogItem = item.id}}>
                <BlogItem focussed={blogItem === item.id} text={item.text}></BlogItem>
            </a>
        </div>
    {/each}
</div>


<style>
    a {
        text-decoration: none;
        color: black;
    }

    #timeline {
        position: absolute;
        left: 50%;

        transform: translateX(50%);

        height: 100%;
        width: 0;
        border: thick solid black;
    }

    .date {
        position: absolute;
        transform: translate(-50%, -150%);
        display: block;
        text-align: center;
        border: thick solid blue;
        border-radius: 50%;

        width: 2rem;
        height: 2rem;

        background-color: white;
    }

    .text {
        position: relative;
        top: -72px;
        left: 20px;
    }

    .blogItem {

    }
</style>