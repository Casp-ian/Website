<script>
  export let imgOn = "https://skyandtelescope.org/wp-content/uploads/Seronik-Full-Moon-1-1-800x600.jpg";
  export let imgOff = "https://thumbs.dreamstime.com/b/smiling-sun-illustration-character-34434947.jpg";

  let on = false;

  let img = on ? imgOn : imgOff;
</script>


<img src={img} width="100" height="100" alt="toggle" on:click={() => on = !on}>

