<script>
    import PrettyLink from "../components/PrettyLink.svelte";
</script>


<div id="wrapper">
  <h1>Tagged Files</h1>
  <p>I dont really feel like rewriting the readme so you can read it on github.</p>
  <p>Maybe when i rewrite it i will post what i learned here.</p>
  <PrettyLink url="https://www.youtube.com/watch?v=wTQeMkYRMcw">Another project</PrettyLink>
  <PrettyLink url="https://codeberg.org/logistic-bot/tbfs">And another one</PrettyLink>
  <PrettyLink url="https://github.com/Casp-ian/TaggedFiles">source code here.</PrettyLink>
</div>


<style>
  #wrapper {
    background-color: var(--back-color);

    margin: .3rem;
    border-radius: .3rem;
    padding: 1rem 2rem 1rem 2rem;
  }
</style>
