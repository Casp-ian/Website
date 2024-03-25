const months = ["January","February","March","April","May","June","July", "August","September","October","November","December"];

const format = {

    date: (epoch) => {
        const date = new Date(epoch);
        return date.getDate() + " " + months[date.getMonth()];
    },

    dateShort: (epoch) => {
        const date = new Date(epoch);
        return date.getDate() + " " + months[date.getMonth()].substring(0,3);
    },
}

export default format;
