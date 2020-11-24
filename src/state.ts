import { writable, Writable } from "svelte/store";
import { v1 as uuidv1 } from 'uuid';

import type { RequestModel } from "./types";

export const requests: Writable<RequestModel[]> = writable([
    { id: uuidv1(), name: 'Gray Kittens', method: 'GET', url: 'https://jsonplaceholder.typicode.com/comments', requestBody: null, isLoading: false },
    { id: uuidv1(), name: 'A Space Rocket', method: 'GET', url: 'http://google.com', requestBody: null, isLoading: false },
    { id: uuidv1(), name: '100 Pounds of Gravel', method: 'GET', url: 'http://google.com', requestBody: null, isLoading: false },
    { id: uuidv1(), name: 'All of the Shrimp', method: 'POST', url: 'http://google.com', requestBody: null, isLoading: false },
    { id: uuidv1(), name: 'A Planet with a Mall', method: 'GET', url: 'http://google.com', requestBody: null, isLoading: false },
]);

export const activeRequest: Writable<RequestModel | null> = writable(null);
