<script>
    import PrettyLink from "../components/PrettyLink.svelte";
</script>


<div id="wrapper">
  <h1>This website</h1>
  
  <p>I made this website to show of some cool projects i wanted to make, but the first project you would see would be the site itself.</p>
  <p>Written in svelte, with some parts of web assembly in rust, and for now no backend.</p>
  <PrettyLink url="https://github.com/Casp-ian/Website">The source code is available on my github</PrettyLink>
</div>

<style>
  #wrapper {
    background-color: var(--back-color);

    margin: .3rem;
    border-radius: .3rem;
    padding: 1rem 2rem 1rem 2rem;
  }
</style>
