import { writable } from 'svelte/store';

let x = 0;
let y = 0;
let velX = 5;
let velY = 5;

let block = (canvas: HTMLCanvasElement) => {
    x += velX;
    y += velY;

    let ctx = canvas.getContext("2d")!;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    let text = "Fuck you";

    ctx.font = "90px serif";
    ctx.fillStyle = "red";


    ctx.fillText(text, x, y);

    let measurement = ctx.measureText(text);

    if (x < 0) {
        velX = Math.abs(velX);
    }
    else if (x + measurement.width > canvas.width) {
        velX = -Math.abs(velX);
    }

    if (y < measurement.actualBoundingBoxAscent) {
        velY = Math.abs(velY);
    }
    else if (y > canvas.height) {
        velY = -Math.abs(velY);
    }
};

export const updateFunction = writable(block);
