import { AxiosError } from "axios";
import {
  UseMutationOptions,
  UseMutationResult,
  useMutation,
} from "react-query";

import { useAxiosClient } from "./AxiosContext";
import { PostOauthAuthorizeReq, PostOauthAuthorizeResp } from "./HttpTypes";

type MutationRet<T, Ret = void> = UseMutationResult<
  Ret,
  AxiosError,
  T,
  undefined
>;
type MutationOption<T, Ret = void> = Omit<
  UseMutationOptions<Ret, AxiosError, T, undefined>,
  "mutationFn"
>;

export function useOauthAuthorizeMutation(
  options?: MutationOption<PostOauthAuthorizeReq, PostOauthAuthorizeResp>
): MutationRet<PostOauthAuthorizeReq, PostOauthAuthorizeResp> {
  const client = useAxiosClient();
  return useMutation(async (payload: PostOauthAuthorizeReq) => {
    const resp = await client.post<PostOauthAuthorizeResp>(
      `/api/oauth/authorize`,
      payload
    );
    return resp.data;
  }, options);
}
