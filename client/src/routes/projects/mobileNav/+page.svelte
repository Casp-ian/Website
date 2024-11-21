<script lang="ts">
	import Image from '$assets/mobile_ranges.png';
	import PrettyImage from '$components/PrettyImage.svelte';
</script>

<h1>Better navbar on mobile</h1>
<p>
  Navigation on mobile often is a little cumbersome, as the navbar is still at the top of the screen.
  You often have to stretch your thumb, or change your grip to press on a teensy tiny button at the top of your screen.
</p>

<div id="sideBySide">
  <div>
    <p> 
      Here is a simple representation of the "optimal" touch area for a right handed user.
      And the standard top of the screen nav is right in the worst spot.
      So there is only one obvious thing to do, move it to the most optimal area...  
      This however obstructs the page content quite a bit, so what if its scrollable, so you can get it out of the way when you read, and swipe it open when you want to get somewhere.
    </p>
    <p>
      There are still things im considering, like subnavs for nested pages, or just repurposing it for function buttons.
    </p>
    <p>
      You can open this website on your phone right now to try it out.
    </p>
    <p>
      Although it isnt done yet...
    </p>
  </div>
  <PrettyImage src={Image} alt="image describing comfortable thum positions" />
</div>




<style>
  #sideBySide {
    display: flex;
    flex-direction: row;
  }

  #sideBySide>div {
    width: 200%;
  }
  
</style>
