import type {HttpResponse} from "./tauri/http";

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

export interface RequestModel {
    id: string;
    name?: string;
    method: Method;
    url: string;
    requestBody?: string;
    isLoading: boolean;
    lastResponse?: HttpResponse;
}
