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

export interface ParamTableRow {
    key: string;
    value: string;
    description: string;
}

////////////////////////////////
// Request types

export type RequestBodyNone = {
    type: "none";
}

export type RequestBodyForm = {
    type: "form-data";
    rows: ParamTableRow[];
}

export type RequestBodyFormUrl = {
    type: "x-www-form-urlencoded";
    rows: ParamTableRow[];
}

export type RequestBodyRaw = {
    type: "raw";
    body: string;
    mode: string;
}

export type RequestBodyBinary = {
    type: "binary";
    filePath: string;
}

// TODO: Use enum.
export const requestTypeKeys = ['none', 'form-data', 'x-www-form-urlencoded', 'raw', 'binary'];

export type RequestTypes =
    RequestBodyNone
    | RequestBodyForm
    | RequestBodyFormUrl
    | RequestBodyRaw
    | RequestBodyBinary

export type RequestBody = {
    formData: ParamTableRow[];
    raw: string;
    filePath: string;
}

export interface RequestState {
    method: Method;
    url: string;
    id: string;
    name: string;
    activeRequestBody: string;
    requestBodies: Record<string, RequestTypes>;
    params: ParamTableRow[];
    headers: ParamTableRow[];
    requestResponseDirection: RequestResponseDirection;
    activeRequestEditor: ActiveRequestEditorTab;
    isLoading: boolean;
    lastResponse?: HttpResponse;
}

export function newRequestState(method: Method, url: string, params: Partial<RequestState> = {}): RequestState {
    const formData: ParamTableRow[] = [{key: '', value: '', description: ''}];
    const requestBodies: Record<string, RequestTypes> = {
        'none': {type: 'none'},
        'form-data': {type: 'form-data', rows: formData},
        'x-www-form-urlencoded': {type: 'x-www-form-urlencoded', rows: formData},
        'raw': {type: 'raw', body: '', mode: 'plain'},
        'binary': {type: 'binary', filePath: ''},
    };

    return {
        id: v1(),
        method,
        url,
        isLoading: false,
        name: '',
        activeRequestBody: 'none',
        requestBodies,
        params: [{key: '', value: '', description: ''}],
        headers: [{key: '', value: '', description: ''}],
        requestResponseDirection: RequestResponseDirection.Request,
        activeRequestEditor: ActiveRequestEditorTab.Params,
        ...params,
    }
}
