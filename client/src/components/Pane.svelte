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

      box-shadow: 0 0 5px 5px black;
      border-radius: 2px;

      background: #C5BFD9;

      pointer-events: auto;
      margin: 5vh 15vw;
      width: 70vw;
    }
</style>