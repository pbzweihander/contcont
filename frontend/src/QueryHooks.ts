import { AxiosError, AxiosInstance, isAxiosError } from "axios";
import { UseQueryResult, useQuery } from "react-query";

import { useAxiosClient } from "./AxiosContext";
import {
  ArtMetadata,
  GetOpenedResp,
  Literature,
  User,
  Vote,
} from "./HttpTypes";

async function get<T>(
  client: AxiosInstance,
  url: string,
  params?: any
): Promise<T | undefined> {
  try {
    const resp = await client.get<T>(url, {
      params,
    });
    return resp.data;
  } catch (error) {
    if (isAxiosError(error)) {
      if (error.response?.status === 404) {
        return undefined;
      }
    }
    throw error;
  }
}

export function useContestName(): UseQueryResult<string, AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contest/name"], async () => {
    const resp = await get<string>(client, "/api/contest/name");
    return resp;
  });
}

export function useUserFromApi(): UseQueryResult<User, AxiosError> {
  const client = useAxiosClient();
  return useQuery(
    ["user"],
    async () => {
      return await get<User>(client, "/api/user");
    },
    { retry: false }
  );
}

export function useSubmissionOpened(): UseQueryResult<
  GetOpenedResp,
  AxiosError
> {
  const client = useAxiosClient();
  return useQuery(["contest/submission/opened"], async () => {
    return await get<GetOpenedResp>(client, "/api/contest/submission/opened");
  });
}

export function useVotingOpened(): UseQueryResult<GetOpenedResp, AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contest/voting/opened"], async () => {
    return await get<GetOpenedResp>(client, "/api/contest/voting/opened");
  });
}

export function useLiterature(
  id: number
): UseQueryResult<Literature, AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contest/literature", id], async () => {
    return await get<Literature>(client, `/api/contest/literature/${id}`);
  });
}

export function useLiteratures(): UseQueryResult<Literature[], AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contest/literatures"], async () => {
    return await get<Literature[]>(client, "/api/contest/literature");
  });
}

export function useArtMetadata(
  id: number
): UseQueryResult<ArtMetadata, AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contest/art/metadata", id], async () => {
    return await get<ArtMetadata>(client, `/api/contest/art/metadata/${id}`);
  });
}

export function useArtMetadatas(): UseQueryResult<ArtMetadata[], AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contest/art/metadatas"], async () => {
    return await get<ArtMetadata[]>(client, "/api/contest/art/metadata");
  });
}

export function useLiteratureVote(
  user: User | undefined,
  id: number
): UseQueryResult<Vote, AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contestt/voting/literature", user, id], async () => {
    if (user == null) {
      return undefined;
    }
    return await get<Vote>(client, `/api/contest/voting/literature/${id}`);
  });
}

export function useArtVote(
  user: User | undefined,
  id: number
): UseQueryResult<Vote, AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contestt/voting/art", user, id], async () => {
    if (user == null) {
      return undefined;
    }
    return await get<Vote>(client, `/api/contest/voting/art/${id}`);
  });
}
