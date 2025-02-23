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

const projects: SubRoute[] = [
    {
        name: 'This',
        icon: '/src/assets/icons/pin.svg',
        path: '/projects/this',
    },
    {
        name: 'Cat',
        icon: '/src/assets/icons/cat.svg',
        path: '/projects/cat',
    },
    {
        name: 'ImageToText',
        icon: '/src/assets/icons/image.svg',
        path: '/projects/imageToText',
    },
];

export const pages: Route[] = [
    {
        name: 'Home',
        icon: '/src/assets/icons/home.svg',
        path: '/home',
        children: [],
    },
    {
        name: 'Settings',
        icon: '/src/assets/icons/sliders.svg',
        path: '/settings',
        children: [],
    },
    {
        name: 'Others',
        icon: '/src/assets/icons/sparks.svg',
        path: '/others',
        children: [],
    },
    {
        name: 'Projects',
        icon: '/src/assets/icons/dashboard.svg',
        path: '/projects',
        children: projects,
    },
];
