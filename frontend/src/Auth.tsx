import { isAxiosError } from "axios";
import { createContext, ReactNode, useContext } from "react";
import { Navigate } from "react-router-dom";

import { User } from "./HttpTypes";
import { useUserFromApi } from "./QueryHooks";

const UserContext = createContext<User>(undefined!);
export const useUser = (): User => useContext(UserContext);

export function AuthRequired({ children }: { children: ReactNode }) {
  const { data: user, isLoading, error } = useUserFromApi();

  if (isLoading) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  if (error) {
    if (isAxiosError(error)) {
      const status = error.response?.status;
      if (status === 401) {
        return <Navigate to="/login" />;
      } else if (status === 403) {
        return <div>Forbidden</div>;
      }
    }
  }

  if (!user) {
    return <Navigate to="/login" />;
  }

  return <UserContext.Provider value={user}>{children}</UserContext.Provider>;
}
