<script lang="ts">
    import {page} from '$app/state';
    import {pages} from "$lib/navigation";
    import {browser} from '$app/environment';

    function isActive(path: string): boolean {
        return page.url.pathname.includes(path);
    }

    let centerX = 0;
    let centerY = 0;
    const radius = 300;

    let distance = $state(0); // total number representing how far the nav is scrolled
    let touchDiff = 0; // movement between last 2 touch events

    if (browser) {
        // note this should be recalculated on resize
        centerX = window.innerWidth - 32;
        centerY = window.innerHeight - 32;
    }

    let getX = $derived((index: number) => {
        let angle = distance / 100 + index * 100;
        return radius * Math.sin(angle) + centerX;
    });

    let getY = $derived((index: number) => {
        let angle = distance / 100 + index * 100;
        return radius * -Math.cos(angle) + centerY;
    });

    let previousTouch: Touch;

    function scrollStart(event: TouchEvent) {
        // reset momentum
        clearInterval(momentumInterval);
        touchDiff = 0;

        previousTouch = event.changedTouches[0];
    }

    function scroll(event: TouchEvent) {
        touchDiff = (
            (previousTouch.screenX - event.changedTouches[0].screenX) +
            (event.changedTouches[0].screenY - previousTouch.screenY)
        ) / 3;
        previousTouch = event.changedTouches[0];
        distance -= touchDiff;
    }

    function scrollEnd(_event: TouchEvent) {
        doMomentum(touchDiff / 2);
    }

    const momentumMinimum = 3;
    const maxVelocity = 15;
    const momentumFramerate = 10; //interval speed
    const momentumFalloff = 0.92;
    let momentumInterval: number | undefined;
    let momentumSpeed: number;
    function doMomentum(startSpeed: number) {

        if (startSpeed > maxVelocity) {
            momentumSpeed = maxVelocity;
        } else if (startSpeed < -maxVelocity) {
            momentumSpeed = -maxVelocity;
        } else {
            momentumSpeed = startSpeed;
        }

        clearInterval(momentumInterval)
        momentumInterval = setInterval(() => {
            console.log(momentumSpeed);
            if (Math.abs(momentumSpeed) <= momentumMinimum) {
                clearInterval(momentumInterval);
            }

            distance -= momentumSpeed;
            momentumSpeed = momentumSpeed * momentumFalloff;
        }, momentumFramerate)
    }
</script>

<nav id="wrapper">
    <button
            ontouchstart={scrollStart}
            ontouchmove={scroll}
            ontouchend={scrollEnd}
            onclick={() => doMomentum(-15)}
            style:left="{getX(-1)}px" style:top="{getY(-1)}px"
    >
        &gt
    </button>
    {#each pages as page, i}
        <a href="{page.path}"
           ontouchstart={scrollStart}
           ontouchmove={scroll}
           ontouchend={scrollEnd}
           style:left="{getX(i)}px" style:top="{getY(i)}px">
            <img src="{page.icon}" alt="{page.name}">
        </a>
    {/each}
    <button
            ontouchstart={scrollStart}
            ontouchmove={scroll}
            ontouchend={scrollEnd}
            onclick={() => doMomentum(15)}
            style:left="{getX(pages.length)}px" style:top="{getY(pages.length)}px"
    >
        &lt
    </button>
</nav>


<style lang="scss">

  #wrapper {
    pointer-events: none;
    position: absolute;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }


  button {
    pointer-events: auto;
    border: none;
    height: 32px;
    width: 32px;
    border-radius: 16px;
    position: absolute;
    background-color: red;
  }

  a {
    pointer-events: auto;
    height: 32px;
    width: 32px;
    border-radius: 16px;
    position: absolute;
    background-color: red;
  }

  img {
    height: 32px;
    width: 32px;
  }
</style>