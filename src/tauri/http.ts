import { promisified } from 'tauri/api/tauri';
import { v1 } from "uuid";

export interface HttpRequest {
    /// The request method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT or TRACE)
    method: string;
    /// The request URL
    url: string;
    /// The request query params
    params: string[][];
    /// The request headers
    headers: string[][];
    /// The request body
    body?: string;

    // TODO: Implement the rest of this.
    /// Whether to follow redirects or not
    followRedirects?: boolean;
    /// Max number of redirections to follow
    maxRedirections?: number;
    /// Connect timeout for the request
    connectTimeout?: number;
    /// Read timeout for the request
    readTimeout?: number;
    /// Timeout for the whole request
    timeout?: number;
    /// Whether the request will announce that it accepts compression
    allowCompression?: boolean;
}

export class Headers {
    private readonly headers: string[][];

    constructor(headers: string[][]) {
        this.headers = headers;
    }

    all(): string[][] {
        return this.headers;
    }

    find(name: string): string[] {
        return this.headers.filter(h => h[0] === name).map(h => h[1]);
    }

    get(name: string): string | null {
        const row = this.headers.find(h => h[0] === name);
        if (!row) return null;
        return row[1];
    }
}

export interface HttpResponse {
    requestId: string;
    body?: string;
    /// Headers are returned as an array of arrays, with [['key', 'value'], ...]
    headers: Headers;
    url: string;
    statusCode: number;
    statusLine: string;
}

interface HttpRequestDTO {
    requestId: string;
    /// The request method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT or TRACE)
    method: string;
    /// The request URL
    url: string;
    /// The request query params
    params: string[][];
    /// The request headers
    headers: string[][];
    /// The request body, base64 encoded.
    body?: string;

    // TODO: Implement the rest of this.
    /// Whether to follow redirects or not
    follow_redirects?: boolean;
    /// Max number of redirections to follow
    max_redirections?: number;
    /// Connect timeout for the request
    connect_timeout?: number;
    /// Read timeout for the request
    read_timeout?: number;
    /// Timeout for the whole request
    timeout?: number;
    /// Whether the request will announce that it accepts compression
    allow_compression?: boolean;
}

export interface HttpResponseDTO {
    requestId: string;
    // Base64 encoded
    body?: string;
    /// Headers are returned as an array of arrays, with [['key', 'value'], ...]
    headers: string[][];
    url: string;
    status_code: number;
    status_line: string;
}

export async function request(request: HttpRequest): Promise<HttpResponse> {
    const dto: HttpRequestDTO = {
        ...request,
        requestId: v1(),
        follow_redirects: request.followRedirects,
        max_redirections: request.maxRedirections,
        connect_timeout: request.connectTimeout,
        read_timeout: request.readTimeout,
        allow_compression: request.allowCompression,
    };

    const response = await promisified<HttpResponseDTO>({
        cmd: 'reposeHttpRequest',
        request: dto,
    });

    const headers = new Headers(response.headers);

    return {
        ...response,
        headers,
        // TODO: Handle binary responses.
        body: response.body ? atob(response.body) : null,
        statusCode: response.status_code,
        statusLine: response.status_line,
    }
}
