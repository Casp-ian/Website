// NOTE: this is kind of a pain if we need to import every icon to make svelte-kit work
import pin from '$assets/icons/pin.svg';
import cat from '$assets/icons/cat.svg';
import image from '$assets/icons/image.svg';
import home from '$assets/icons/home.svg';
import sliders from '$assets/icons/sliders.svg';
import sparks from '$assets/icons/sparks.svg';
import dashboard from '$assets/icons/dashboard.svg';


export type SubRoute = {
    name: string,
    icon: string,
    path: string,
}

export type Route = {
    name: string,
    icon: string,
    path: string,
    children: SubRoute[],
}

export const projects: SubRoute[] = [
    {
        name: 'This',
        icon: pin,
        path: '/projects/this',
    },
    {
        name: 'Cat',
        icon: cat,
        path: '/projects/cat',
    },
    {
        name: 'ImageToText',
        icon: image,
        path: '/projects/imageToText',
    },
];

export const pages: Route[] = [
    {
        name: 'Home',
        icon: home,
        path: '/home',
        children: [],
    },
    {
        name: 'Settings',
        icon: sliders,
        path: '/settings',
        children: [],
    },
    {
        name: 'Others',
        icon: sparks,
        path: '/others',
        children: [],
    },
    {
        name: 'Projects',
        icon: dashboard,
        path: '/projects',
        children: projects,
    },
];
