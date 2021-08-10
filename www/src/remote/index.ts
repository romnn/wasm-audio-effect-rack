import {Location} from "history";
import {match} from "react-router-dom";

import {
  InstanceId,
} from "../generated/proto/grpc/remote_pb";

import {StreamAuthInterceptor, UnaryAuthInterceptor} from "./interceptors";

export interface RemoteState {
  session?: string;
  instance?: string;
}
;

export interface RemoteURLQueryProps {
  session?: string;
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
  public session?: string;
  public instance?: string;

  public endpoint =
      window.location.protocol + "//" + window.location.hostname + ":9000";

  protected client: C;
  protected interceptors: {
    stream: StreamAuthInterceptor,
    unary: UnaryAuthInterceptor,
  };

  // public static generateToken = (n = 10):
  //     string => {
  //       const chars =
  //           'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
  //       var token = '';
  //       for (var i = 0; i < n; i++) {
  //         token += chars[Math.floor(Math.random() * chars.length)];
  //       }
  //       return token;
  //     }

  public static getSessionInstance =
      (match: match<{session?: string, instance?: string}>,
       location: Location<any>):
          {session?: string, instance?: string} => {
            const queryParams = new URLSearchParams(location.search);
            const session =
                match.params.session ?? queryParams.get("session") ?? undefined;
            const instance =
                match.params.instance ?? queryParams.get("instance")
                ?? undefined;
            return {session, instance};
          }

  constructor(client: GrpcClientFactory<C>, session: string|undefined,
              instance: string|undefined) {
    this.session = session;
    this.instance = instance;
    this.interceptors = {
      stream : new StreamAuthInterceptor(this.session, this.instance),
      unary : new UnaryAuthInterceptor(this.session, this.instance),
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
