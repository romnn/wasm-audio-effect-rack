import {Location} from "history";
import {match} from "react-router-dom";

export interface P2PState {
  token?: string;
};

export interface P2PURLQueryProps {
  token?: string;
}

export const getToken = (match: match<{token?: string}>,
                         location: Location<any>): string|undefined => {
  const queryParams = new URLSearchParams(location.search);
  return match.params.token ?? queryParams.get("token") ?? undefined;
};
