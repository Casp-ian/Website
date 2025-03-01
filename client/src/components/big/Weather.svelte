<script lang="ts">
    import {onDestroy, onMount} from 'svelte';
    import {weather} from '$stores/weather';
    import {afterNavigate} from "$app/navigation";

    let generatorDelay = 400;
    let maxParticles = 20;

    let precipitants: { character: string; x: number; y: number }[] = [];

    let width = 0;
    let height = 0;

    const recalculateDimensions = () => {
        width = Math.max(document.body.clientWidth, window.innerWidth);
        height = Math.max(document.body.clientHeight, window.innerHeight);
    }

    let interval: number;
    onMount(async () => {
        // TODO maybe just get rid of the interval when weather == 'none', dont think it has too much impact tho
        // tho it could be neat to have a specific `spawnSnow()` and `moveSnow()` and so forth
        interval = setInterval(() => {
            spawnPrecipitation();
            movePrecipitation();
            clearPrecipitation();
        }, generatorDelay);

        recalculateDimensions();

        window.addEventListener("resize", () => {
            precipitants = [];
            recalculateDimensions();
        })
    });

    afterNavigate(async () => {
        precipitants = [];
        recalculateDimensions();
    })

    onDestroy(async () => {
        clearInterval(interval);
    });

    // clear particles when weather changes
    weather.subscribe(() => {
        precipitants = [];
    });

    function spawnPrecipitation() {
        if ($weather === 'none') {
            return;
        }

        let character = {
            character: '+',

            // todo make these adjustments more exact
            x: Math.floor(Math.random() * (width - 20)), // adjust for character width
            y: Math.floor(Math.random() * (height - 40)), // adjust for character height
        };

        precipitants.push(character);

        precipitants = precipitants;
    }

    function movePrecipitation() {
        if ($weather === 'none') {
            return;
        }

        precipitants.forEach((element) => {
            let i = Math.round(Math.random() * 3);
            switch (i) {
                case 0:
                    element.character = 'x';
                    break;
                case 1:
                    element.character = '+';
                    break;
                case 2:
                    element.character = '*';
                    break;
            }
        });
    }

    function clearPrecipitation() {
        if ($weather === 'none') {
            return;
        }

        if (precipitants.length > maxParticles) {
            precipitants.shift();
        }
    }
</script>


<!-- TODO: this means particles cant spawn beneath the initial viewport, like when you scroll down -->
<!--<svelte:window bind:innerWidth={width} bind:innerHeight={height} />-->

{#each precipitants as precipitant}
    <p class="particle" style="left: {precipitant.x}px; top: {precipitant.y}px;">
        {precipitant.character}
    </p>
{/each}


<style>
    /*
        TODO: when particles can spawn underneath the current viewport
        clip particles outside of what is supposed to be the dimensions of the page,
        might be harder than expected because the particles also influence the dimensions
    */
    .particle {
        color: white;
        position: absolute;
        user-select: none;
        pointer-events: none;
    }
</style>
