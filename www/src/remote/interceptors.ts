import {ClientReadableStream, Metadata, Request, UnaryResponse} from "grpc-web";

export interface AuthInterceptor {
  token: string;
}

export class UnaryAuthInterceptor implements AuthInterceptor {
  public token: string;
  constructor(token: string) { this.token = token; }
  intercept<REQ, RESP>(request: Request<REQ, RESP>,
                       invoker: (request: Request<REQ, RESP>) =>
                           Promise<UnaryResponse<REQ, RESP>>):
      Promise<UnaryResponse<REQ, RESP>> {
    // Update the request metdata before the RPC.
    const md = request.getMetadata() as Metadata;
    md["user-token"] = this.token;
    return invoker(request);
  }
}

export class StreamAuthInterceptor implements AuthInterceptor {
  public token: string;
  constructor(token: string) { this.token = token; }

  intercept<REQ, RESP>(
      request: Request<REQ, RESP>,
      invoker: (request: Request<REQ, RESP>) => ClientReadableStream<RESP>):
      ClientReadableStream<RESP> {
    // Update the request metdata before the RPC.
    const md = request.getMetadata() as Metadata;
    md["user-token"] = this.token;
    return invoker(request);
  }
}
