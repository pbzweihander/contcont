import { AxiosError, AxiosInstance, isAxiosError } from "axios";
import { UseQueryResult, useQuery } from "react-query";

import { useAxiosClient } from "./AxiosContext";
import { User } from "./HttpTypes";

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
      const resp = await get<User>(client, "/api/user");
      return resp;
    },
    { retry: false }
  );
}
