<script>
    import PrettyLink from "./PrettyLink.svelte";

    import githubImage from "../assets/github-mark.svg";
    import youtubeImage from "../assets/yt_icon_rgb.png";
</script>

<hr>
<div id="links">
    <PrettyLink url="https://github.com/Casp-ian" img={githubImage}>
        github
    </PrettyLink>
    <PrettyLink url="https://www.youtube.com/watch?v=dQw4w9WgXcQ" img={youtubeImage}>
        i have no other links :)
    </PrettyLink>
</div>

<style>
    #links {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-evenly;
    }
</style>
