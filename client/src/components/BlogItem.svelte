<script>
    import { marked } from 'marked'

    export let text = "No text...";
    export let focussed = false;

    function truncate(string, length) {
        return string.substring(0, length) + '...';
    }
</script>


<div id="blogItem">
    <hr>
    <div id="text" class={focussed ? "" : "shrunk"}>
        {@html marked(text)}
    </div>
</div>


<style>
    #blogItem {
    }

    #pointer {
        margin-top: 18px;
    }

    #text {
        overflow: clip;
        margin-right: 3rem;
        margin-left: 3rem;
    }

    .shrunk {
        height: 100px;
    }
</style>