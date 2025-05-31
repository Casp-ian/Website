import {get, writable} from 'svelte/store';

const defaultNotifications: string[] = []
export const notifications = writable(defaultNotifications);

export function push(input: string) {
    let x = get(notifications);
    x.push(input);
    notifications.set(x);
}
