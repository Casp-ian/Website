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

    let copyText = "⎘";
    
    function copy() {
        navigator.clipboard.writeText(text);
        copyText = "✓";
        setTimeout(() => {copyText = "⎘"}, 1000);
    }

</script>

<div id="wrapper">
    <pre id="code"/>
    <div id="copy">
        <button on:click={copy}> 
            {copyText}
        </button>
    </div>
</div>


<style>
    @import url("https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/a11y-dark.min.css");
    #wrapper {
        padding: 10px;
        background-color: black;
        display: flex;
        width: max-content;
        flex-direction: row;
        
    }

    #copy {
        padding: 5px;
        margin-left: 20px;
        width: 1rem;
        height: 1rem;
    }
    
    #code {
        color: white;
    }
</style>
