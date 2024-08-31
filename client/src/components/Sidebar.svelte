<script>
    import Footer from "./Footer.svelte";
    import Header from "./Header.svelte";

    export let active;
  // TODO


</script>


<div id="wrapper">
  <Header active={active}/>
  
  <Footer />
</div>


<style>

  #wrapper {
    display: flex;
    flex-direction: column;
    
    align-content: center;
  }

</style>
