<script>
    import { weather } from '../stores/weather';
</script>

<button on:click={() => {weather.update(() => 'none')}}>none</button>
<button on:click={() => {weather.update(() => 'night')}}>night</button>
<button on:click={() => {weather.update(() => 'snow')}}>snow</button>
