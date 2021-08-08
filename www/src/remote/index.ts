import {Location} from "history";
import {match} from "react-router-dom";
import {StreamAuthInterceptor, UnaryAuthInterceptor} from "./interceptors";

export interface RemoteState {
  token: string;
  instance: string;
}
;

export interface RemoteURLQueryProps {
  token?: string;
  instance?: string;
}

// (window.location.port ? ":" + window.location.port : "");
// export const GRPC_ENDPOINT =
//     window.location.protocol + "//" + window.location.hostname + ":9000";

export interface GrpcClientFactory<C> {
  new(hostname: string, credentials?: null|{ [index: string]: string; },
      options?: null|{ [index: string]: any; }): C;
}

export default class RemoteClient<C> {
  // public abstract userToken: string;
  public userToken: string;

  public endpoint =
      window.location.protocol + "//" + window.location.hostname + ":9000";

  protected client: C;
  protected interceptors: {
    stream: StreamAuthInterceptor,
    unary: UnaryAuthInterceptor,
  };

  public static generateToken = (n = 10):
      string => {
        const chars =
            'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
        var token = '';
        for (var i = 0; i < n; i++) {
          token += chars[Math.floor(Math.random() * chars.length)];
        }
        return token;
      }

  public static getUser = (match: match<{token?: string, instance?: string}>,
                           location: Location<any>):
      {token?: string, instance?: string} => {
        const queryParams = new URLSearchParams(location.search);
        const token =
            match.params.token ?? queryParams.get("token") ?? undefined;
        const instance =
            match.params.instance ?? queryParams.get("instance") ?? undefined;
        return {token, instance};
      }

  constructor(client: GrpcClientFactory<C>, userToken: string) {
    this.userToken = userToken;
    this.interceptors = {
      stream : new StreamAuthInterceptor(this.userToken),
      unary : new UnaryAuthInterceptor(this.userToken),
    };
    this.client = new client(this.endpoint, null, {
      unaryInterceptors : [ this.interceptors.unary ],
      streamInterceptors : [ this.interceptors.stream ]
    });
  }
}

// export default abstract class RemoteClient {
//   public abstract userToken: string;

//   protected abstract interceptors: {
//     stream: StreamAuthInterceptor,
//     unary: UnaryAuthInterceptor,
//   };
// }
