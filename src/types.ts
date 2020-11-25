import type {HttpResponse} from "./tauri/http";
import {v1} from "uuid";

export type Method =
    'GET'
    | 'POST'
    | 'PUT'
    | 'PATCH'
    | 'DELETE'
    | 'HEAD'
    | 'OPTIONS';

export const methods: Method[] = [
    'GET',
    'POST',
    'PUT',
    'PATCH',
    'DELETE',
    'HEAD',
    'OPTIONS'
];

export enum RequestResponseDirection { Request, Response }

export enum ActiveRequestEditorTab {
    Params = 'Params',
    Headers = 'Headers',
    Body = 'Body',
}

export enum ActiveRequestBodyTab {
    None = "None",
    Raw = "Raw"
}


export interface RequestState {
    method: Method;
    url: string;
    id: string;
    name?: string;
    requestBody?: string;
    requestResponseDirection: RequestResponseDirection;
    activeRequestEditor: ActiveRequestEditorTab;
    activeRequestBody: ActiveRequestBodyTab;
    isLoading: boolean;
    lastResponse?: HttpResponse;
}

export function newRequestState(method: Method, url: string, params: Partial<RequestState> = {}): RequestState {
    return {
        id: v1(),
        method,
        url,
        isLoading: false,
        requestResponseDirection: RequestResponseDirection.Request,
        activeRequestEditor: ActiveRequestEditorTab.Params,
        activeRequestBody: ActiveRequestBodyTab.None,
        ...params,
    }
}

// export class RequestState {
//     constructor(
//         public method: Method,
//         public url: string,
//         public id: string = v1(),
//         public name?: string,
//         public requestBody?: string,
//         public requestResponseDirection: RequestResponseDirection = RequestResponseDirection.Request,
//         public activeRequestEditor: ActiveRequestEditorTab = ActiveRequestEditorTab.Params,
//         public activeRequestBody: ActiveRequestBodyTab = ActiveRequestBodyTab.None,
//         public isLoading: boolean = false,
//         public lastResponse?: HttpResponse
//     ) { }
//
//     equal(other: RequestState): boolean {
//         return this.id === other.id;
//     }
// }
