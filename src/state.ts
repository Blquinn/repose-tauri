import { writable, Writable } from "svelte/store";

import type { RequestModel } from "./types";

export const requests: Writable<RequestModel[]> = writable([
    { name: 'Gray Kittens', method: 'POST', url: 'http://google.com', requestBody: null },
    { name: 'A Space Rocket', method: 'GET', url: 'http://google.com', requestBody: null },
    { name: '100 Pounds of Gravel', method: 'GET', url: 'http://google.com', requestBody: null },
    { name: 'All of the Shrimp', method: 'GET', url: 'http://google.com', requestBody: null },
    { name: 'A Planet with a Mall', method: 'GET', url: 'http://google.com', requestBody: null },
]);
export const activeRequest: Writable<RequestModel | null> = writable(null);
