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
        name: 'this',
        icon: '/src/assets/icons/pin.svg',
        path: '/projects/this',
    },
    {
        name: 'cat',
        icon: '/src/assets/icons/cat.svg',
        path: '/projects/cat',
    },
    {
        name: 'imageToText',
        icon: '/src/assets/icons/image.svg',
        path: '/projects/imageToText',
    },
];

export const pages: Route[] = [
    {
        name: 'home',
        icon: '/src/assets/icons/home.svg',
        path: '/',
        children: [],
    },
    {
        name: 'settings',
        icon: '/src/assets/icons/sliders.svg',
        path: '/settings',
        children: [],
    },
    {
        name: 'projects',
        icon: '/src/assets/icons/dashboard.svg',
        path: '/projects',
        children: projects,
    },
];
