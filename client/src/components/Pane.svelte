<script lang="ts">
    import type { Snippet } from 'svelte'

    let { children }: { children: Snippet } = $props()
</script>


<div>
    {@render children()}
</div>


<style lang="scss">
    div {
      border-radius: 50px;

      //background: #C5BFD9;
      backdrop-filter: blur(2px) brightness(120%);

      margin: 5vh 0;
    }
</style>