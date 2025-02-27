<script lang="ts">
    import type { Snippet } from 'svelte'
    let { children }: { children: Snippet } = $props()
</script>


<div>
    {@render children()}
</div>


<style lang="scss">
    div {
      overflow: auto;

      box-shadow: 0 0 5px 5px white;
      border-radius: 2px;

      background: white;

      pointer-events: auto;
      margin: 5vh 15vw;
      width: 70vw;
    }
</style>