import { request, HttpResponse } from './tauri/http';

export class Client {
    async request(method: string, url: string): Promise<HttpResponse> {
        return await request({
            method,
            url,
            headers: [],
            params: [],
        });
    }
}

export const defaultClient = new Client();
