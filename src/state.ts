import { writable, Writable } from "svelte/store";

import type { RequestState } from "./types";
import { newRequestState } from "./types";

export const requests: Writable<RequestState[]> = writable([
    newRequestState('GET', 'https://jsonplaceholder.typicode.com/comments', {name: 'Get Comments'}),
    newRequestState('GET', 'http://google.com', {name: 'Get google'}),
    newRequestState('GET', 'http://google.com', {name: 'Get google again and again and again and again and again.'}),
    newRequestState('POST', 'http://google.com'),
    newRequestState('GET', 'http://google.com', {name: 'Get Comments'}),
]);

export const activeRequest: Writable<RequestState | null> = writable(null);

export function updateRequest(req: RequestState) {
    requests.update(reqs => reqs.map(r => r.id === req.id ? req : r));
}

export function setActiveRequest(current: RequestState | null, newRequest: RequestState | null) {
    if (current) updateRequest(current);
    activeRequest.update(_ => newRequest);
}
