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

export interface ParamTableRow {
    key: string;
    value: string;
    description: string;
}

export interface RequestState {
    method: Method;
    url: string;
    id: string;
    name: string;
    requestBody?: string;
    params: ParamTableRow[];
    headers: ParamTableRow[];
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
        name: '',
        params: [{key: '', value: '', description: ''}],
        headers: [{key: '', value: '', description: ''}],
        requestResponseDirection: RequestResponseDirection.Request,
        activeRequestEditor: ActiveRequestEditorTab.Params,
        activeRequestBody: ActiveRequestBodyTab.None,
        ...params,
    }
}
