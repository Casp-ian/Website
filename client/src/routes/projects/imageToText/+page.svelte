<script>
    import PrettyLink from '$components/PrettyLink.svelte';
    import BuffFrog from '$assets/aar/buff_frog_ImageToText_example.png';
    import Powershell from '$assets/aar/powershell.png';
    import PrettyImage from '$components/PrettyImage.svelte';
    import Pane from "$components/Pane.svelte";
</script>

<Pane>

    <h1>First part</h1>

    <p>The code is available <PrettyLink url="https://github.com/Casp-ian/ImageToText">here</PrettyLink></p>

</Pane>
<Pane>
    <p>
        This is a project to turn images into ascii art, I am far from the first to make a project out of this.
        But there are a few features im planning to implement, mainly cool edge detection, and running on the GPU with a
        compute shader.
    </p>

    <p>I chose to write it in Rust with WGPU.</p>

    <p>
        I had a lot of fun adding support for getting the users webcam as input, and making sure it worked on windows as well.
        Below is a picture of me testing the program on a friends laptop.
    </p>
    <PrettyImage margin="1rem" src={Powershell} alt="windows image example"/>

    <h2>
        Current State
    </h2>

    <p>
        I just destroyed part of the current edge detection, and am planning to implement something close to Brian Macintosh's method...
    </p>

    <h2>
        Others
    </h2>

    <p>
        Nothing in this project is a grand new idea, in fact most of it is copied from other peoples projects, ill show
        them here.
    </p>

    <p>
        This
        <PrettyLink url="https://www.youtube.com/watch?v=gg40RWiaHRY">
            video
        </PrettyLink>
        by Acerola is great, and helped me understand almost everything needed for the project.
    </p>
    <p>
        Another
        <PrettyLink url="http://brianmacintosh.com/asciiart/">
            Ascii generator
        </PrettyLink>
        Shows of a different edge detection method and a nice way to turn the edges into various different characters.
    </p>
</Pane>


<style>
    pre, p, h1, h2, h3 {
        padding: 1rem;
        margin: 0;
    }

    pre {
        overflow: hidden;
    }
</style>