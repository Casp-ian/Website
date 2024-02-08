<script>
    // i am so happy i got this to work, might have been a waste of time but now i do have a reusable form component

    import {setContext} from "svelte";

    export let mandatoryFields = [];
    export let method;

    export let disabled = false;

    const data = {}

    function submit() {
        if (!mandatoryFields.every((field) => Object.getOwnPropertyNames(data).includes(field))) {
            alert('Not all fields are filled in correctly.')
            return;
        }
        method(data);
    }

    setContext("update", data)

</script>


<slot/>

<button on:click={submit} {disabled}>submit</button>