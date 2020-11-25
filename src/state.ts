import { writable, Writable } from "svelte/store";

import type { RequestState } from "./types";
import {newRequestState} from "./types";

export const requests: Writable<RequestState[]> = writable([
    newRequestState('GET', 'https://jsonplaceholder.typicode.com/comments'),
    newRequestState('GET', 'http://google.com'),
    newRequestState('GET', 'http://google.com'),
    newRequestState('POST', 'http://google.com'),
    newRequestState('GET', 'http://google.com'),
]);

export const activeRequest: Writable<RequestState | null> = writable(null);

export function setActiveRequest(current: RequestState | null, newRequest: RequestState | null) {
    console.log(current, newRequest);
    if (current) requests.update(reqs => reqs.map(r => r.id === current.id ? current : r));
    activeRequest.update(_ => newRequest);
}
