import {ClientReadableStream, Metadata, Request, UnaryResponse} from "grpc-web";

export interface AuthInterceptor {
  token: string;
}

export const SESSION_TOKEN_KEY = "session-token";
export const INSTANCE_ID_KEY = "instance-id";

export class AuthInterceptor {
  public session?: string;
  public instance?: string;

  constructor(session: string|undefined, instance: string|undefined) {
    this.session = session;
    this.instance = instance;
  }
}

export class UnaryAuthInterceptor extends AuthInterceptor implements
    AuthInterceptor {

  intercept<REQ, RESP>(request: Request<REQ, RESP>,
                       invoker: (request: Request<REQ, RESP>) =>
                           Promise<UnaryResponse<REQ, RESP>>):
      Promise<UnaryResponse<REQ, RESP>> {
    // Update the request metdata before the RPC.
    const md = request.getMetadata() as Metadata;
    if (this.session)
      md[SESSION_TOKEN_KEY] = this.session;
    if (this.instance)
      md[INSTANCE_ID_KEY] = this.instance;
    return invoker(request);
  }
}

export class StreamAuthInterceptor extends AuthInterceptor implements
    AuthInterceptor {
  // public session: string;
  // constructor(token: string) { this.token = token; }

  intercept<REQ, RESP>(
      request: Request<REQ, RESP>,
      invoker: (request: Request<REQ, RESP>) => ClientReadableStream<RESP>):
      ClientReadableStream<RESP> {
    // Update the request metdata before the RPC.
    const md = request.getMetadata() as Metadata;
    if (this.session)
      md[SESSION_TOKEN_KEY] = this.session;
    if (this.instance)
      md[INSTANCE_ID_KEY] = this.instance;
    return invoker(request);
  }
}
