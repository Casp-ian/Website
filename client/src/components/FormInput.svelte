<script>
    import {getContext, setContext} from "svelte";

    export let type = 'text';
    export let name = '';
    export let placeholder = '';

    const update = getContext('update');

    let value = '';
    function typeAction(node) {
        node.type = type;
    }

    export let validation = [
        {
            regex: /./,
            message: "must not be empty"
        }
    ];
    function validate() {

        validationMessage = '';
        validation.forEach((rule) => {
            if (!value.match(rule.regex)) {
                validationMessage =  'violation: ' + rule.message;
            }
        })

        if (validationMessage === '') {
            update[name] = value;
        } else {
            delete update[name]
        }
    }

    function getValue() {
        if (validationMessage === '') {
            return value;
        }
        return null;
    }

    $: setContext('get' + value, getValue);


    let validationMessage = '';
</script>

<p>{validationMessage}</p>
<label>{name}</label>
<input use:typeAction {placeholder} bind:value={value} on:input={validate}>


<style>
    p {
        color: red;
        margin-bottom: 0;
    }
</style>