<script>
    import hljs from 'highlight.js/lib/core';
    import javascript from 'highlight.js/lib/languages/javascript';
    import rust from 'highlight.js/lib/languages/rust';
    hljs.registerLanguage('javascript', javascript);
    hljs.registerLanguage('rust', rust);

    import { onMount } from "svelte";

    onMount(async () => {
        document.getElementById("code").innerHTML = hljs.highlight(text, {language: language}).value;
    })

    export let language;

    export let text;

</script>


<div id="code">
</div>


<style>
    @import url("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css");
    #code {
        padding: 10px;
    }
</style>