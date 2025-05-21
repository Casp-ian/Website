<script lang="ts">
    import type { Snippet } from 'svelte'

    let { children }: { children: Snippet } = $props()
</script>


<div>
    {@render children()}
</div>


<style lang="scss">
    div {
      border-radius: 2rem;

      backdrop-filter: blur(5px) brightness(50%);

      margin: 5vh 0;
    }
</style>