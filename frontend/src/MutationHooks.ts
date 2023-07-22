import { AxiosError } from "axios";
import {
  UseMutationOptions,
  UseMutationResult,
  useMutation,
} from "react-query";

import { useAxiosClient } from "./AxiosContext";
import {
  ArtMetadata,
  Literature,
  PostArtReq,
  PostLiteratureReq,
  PostOauthAuthorizeReq,
  PostOauthAuthorizeResp,
  PostVoteReq,
} from "./HttpTypes";

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
      "/api/oauth/authorize",
      payload
    );
    return resp.data;
  }, options);
}

export function usePostLiteratureMutation(
  options?: MutationOption<PostLiteratureReq, Literature>
): MutationRet<PostLiteratureReq, Literature> {
  const client = useAxiosClient();
  return useMutation(async (payload: PostLiteratureReq) => {
    const resp = await client.post<Literature>(
      "/api/contest/submission/literature",
      payload
    );
    return resp.data;
  }, options);
}

export function usePostArtMutation(
  options?: MutationOption<PostArtReq, ArtMetadata>
): MutationRet<PostArtReq, ArtMetadata> {
  const client = useAxiosClient();
  return useMutation(async (payload: PostArtReq) => {
    const formData = new FormData();
    formData.append("title", payload.title);
    formData.append("description", payload.description);
    formData.append("isNsfw", payload.isNsfw ? "true" : "false");
    formData.append("data", payload.file);
    const resp = await client.post<ArtMetadata>(
      "/api/contest/submission/art",
      formData
    );
    return resp.data;
  }, options);
}

export function usePostLiteratureVoteMutation(
  options?: MutationOption<PostVoteReq, void>
): MutationRet<PostVoteReq, void> {
  const client = useAxiosClient();
  return useMutation(async (payload: PostVoteReq) => {
    await client.post(`/api/contest/voting/literature/${payload.id}`);
  }, options);
}

export function usePostArtVoteMutation(
  options?: MutationOption<PostVoteReq, void>
): MutationRet<PostVoteReq, void> {
  const client = useAxiosClient();
  return useMutation(async (payload: PostVoteReq) => {
    await client.post(`/api/contest/voting/art/${payload.id}`);
  }, options);
}
