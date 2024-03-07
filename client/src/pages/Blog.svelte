<script>
    import BlogItem from "../components/BlogItem.svelte";
    import format from "../lib/format.js";

    export let params;

    export let items = [
        {
            id: "1",
            date: 1706773192213,
            text: "# Hello! \n \
            Hello! \n \
            Hello! \n \
            Hello! \n \
            Hello! \n \
            Hello! \n \
            Hello! \n \
            this is in markdown \n \
            i dont know how to do markdown \n \
            but thats fine",
        },
        {
            id: "2",
            date: 1706472180713,
            text: "# Longererererer title! \n \
            this is in markdown \n \
            this is in markdown \n \
            this is in markdown \n \
            this is in markdown \n \
            this is in markdown \n \
            this is in markdown \n \
            this is in markdown \n \
            this is in markdown \n \
            - bing \n \
            - boing \n \
            i dont know how to do markdown \n \
            but thats fine",
        },
        {
            id: "3",
            date: 1706261080713,
            text: "# Longererererer title! \n \
            this is in markdown \n \
            i dont know how to do markdown \n \
            i dont know how to do markdown \n \
            i dont know how to do markdown \n \
            i dont know how to do markdown \n \
            i dont know how to do markdown \n \
            i dont know how to do markdown \n \
            i dont know how to do markdown \n \
            i dont know how to do markdown \n \
            but thats fine",
        },
    ];

    const now = Date.now();

    function processItems() {
        items = items.sort((a, b) => b.date - a.date);

        const pixelsPerDay = 20

        let previousDate = now;
        for (let i = 0; i < items.length; i++) {
            const thisDate = items[i].date;
            items[i].diff = (previousDate - thisDate) / 86400000 * pixelsPerDay;
            previousDate = thisDate;
        }
    }
    processItems();

    let blogItem = params.id;
    // TODO scroll to blogitem
</script>


<div id={(blogItem === undefined) ? "grid" : ((blogItem % 2 !== 0) ? "gridLeft" : "gridRight")}>

<!--    current date-->
    <a class="date" href={"/blog"} on:click={() => {blogItem = undefined}}>
        {format.dateShort(now)}
    </a>

    {#each items as item, i (i)}
        <div class="timeline" style="--diff: {item.diff}px"></div>

        <a class="date" href={"/blog/" + item.id} on:click={() => {blogItem = item.id}}
           style={"grid-row-start: " + (2 * i + 3)}>
            {format.dateShort(item.date)}
        </a>

        <a class="text {(i % 2 === 0) ? 'left' : 'right'}" href={"/blog/" + item.id} on:click={() => {blogItem = item.id}}
           style="--height: {i}">
            <BlogItem focussed={blogItem === item.id} text={item.text}></BlogItem>
        </a>
    {/each}
</div>


<style>
    #grid {
        margin-top: 2rem;
        display: grid;
        grid-template-columns: calc((100% - (2.4rem + 2px)) / 2) calc(2.4rem + 2px) calc((100% - (2.4rem + 2px)) / 2);
    }
    #gridLeft {
        margin-top: 2rem;
        display: grid;
        grid-template-columns: calc((100% - (4.8rem + 4px))) calc(2.4rem + 2px) calc(2.4rem + 2px);
    }
    #gridRight {
        margin-top: 2rem;
        display: grid;
        grid-template-columns: calc(2.4rem + 2px)calc(2.4rem + 2px) calc((100% - (4.8rem + 4px)));
    }

    a {
        text-decoration: none;
        color: black;
    }

    .date {
        grid-column-start: 2;

        text-align: center;
        border: 2px solid white;
        border-radius: 50%;

        width: 2.4rem;
        height: 2.4rem;

        background-color: lightgray;
    }

    .text {
        grid-row-start: calc(2 * var(--height) + 3);
        /*TODO there has to be a neater way than this*/
        grid-row-end: 9999999999999999;

        overflow: clip;
    }

    .text.left {
        grid-column-start: 1;
    }
    .text.right {
        grid-column-start: 3;
    }

    .timeline {
        background-color: black;
        grid-column-start: 2;
        grid-column-end: 2;
        width: 2px;
        position: relative;
        left: 50%;
        transform: translateX(-50%);

        height: var(--diff);
    }

</style>