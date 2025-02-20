export type Route = {
    name: string,
    icon: string,
    path: string,
    // children: Route[],
}

export const pages: Route[] = [
    {
        name: 'home',
        icon: '/src/assets/icons/home.svg',
        path: '/',
    },
    {
        name: 'projects',
        icon: '/src/assets/icons/dashboard.svg',
        path: '/projects',
    },
];

export const subPages: Route[] = [
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
]

/*
	const mainPaths = {
		home: '/',
		projects: '/projects',
	};

	const projects = {
		this: '/projects/this',
		cat: '/projects/cat',
		// "conway": "/projects/conway",
		// "orbiting balls": "/projects/orbiting_balls",
		'tagged files': '/projects/taggedFiles',
		'image to text': '/projects/imageToText',
		'mobile navigation': '/projects/mobileNav',
		// "tanks": "/projects/tanks",
	};
 */