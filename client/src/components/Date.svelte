<script>
    export let epoch;

    const months = [
        'January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'
    ]

    let text;
    $: {
        const dateDiff = epoch - Date.now();
        if (dateDiff < 0) {
            text = 'already ended'
        } else if (dateDiff < 1000 * 60 * 60) {
            text = 'ends in ' + Math.ceil(dateDiff / 1000 / 60) + ' minutes!';
        } else if (dateDiff < 1000 * 60 * 60 * 24) {
            text = 'ends in ' + Math.ceil(dateDiff / 1000 / 60 / 60) + ' hours!';
        } else {
            const date = new Date(0);
            date.setUTCMilliseconds(epoch);
            text = 'ends ' + date.getDate() + ' ' + months[date.getMonth()];
        }
    }

</script>


<p>{text}</p>