import { AxiosError, AxiosInstance, isAxiosError } from "axios";
import { UseQueryResult, useQuery } from "react-query";

import { useAxiosClient } from "./AxiosContext";
import {
  ArtMetadata,
  GetEnabledResp,
  GetOpenedResp,
  GetResultOpenedResp,
  Literature,
  LiteratureMetadata,
  User,
  Vote,
  WithVoteCount,
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
export function useEnabled(): UseQueryResult<GetEnabledResp, AxiosError> {
  const client = useAxiosClient();
  return useQuery(["contest/enabled"], async () => {
    const resp = await get<GetEnabledResp>(client, "/api/contest/enabled");
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

export function useLiteratureMetadatas(): UseQueryResult<
  LiteratureMetadata[],
  AxiosError
> {
  const client = useAxiosClient();
  return useQuery(["contest/literature/metadatas"], async () => {
    return await get<LiteratureMetadata[]>(
      client,
      "/api/contest/literature/metadata"
    );
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

export function useResultOpened(): UseQueryResult<
  GetResultOpenedResp,
  AxiosError
> {
  const client = useAxiosClient();
  return useQuery(["contest/result/opened"], async () => {
    return await get<GetOpenedResp>(client, "/api/contest/result/opened");
  });
}

export function useLiteratureResults(): UseQueryResult<
  WithVoteCount<LiteratureMetadata>[],
  AxiosError
> {
  const client = useAxiosClient();
  return useQuery(["contest/result/literature"], async () => {
    return await get<WithVoteCount<LiteratureMetadata>[]>(
      client,
      "/api/contest/result/literature"
    );
  });
}

export function useArtResults(): UseQueryResult<
  WithVoteCount<ArtMetadata>[],
  AxiosError
> {
  const client = useAxiosClient();
  return useQuery(["contest/result/art"], async () => {
    return await get<WithVoteCount<ArtMetadata>[]>(
      client,
      "/api/contest/result/art"
    );
  });
}
