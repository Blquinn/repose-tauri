import {request, HttpResponse, HttpRequest} from './tauri/http';

export class Client {
    async request(method: string, url: string, params?: Partial<HttpRequest>): Promise<HttpResponse> {
        const req: HttpRequest = {
            ...params ?? {},
            params: params?.params ?? [],
            headers: params?.headers ?? [],
            url,
            method
        };

        return await request(req);
    }
}

export const defaultClient = new Client();
