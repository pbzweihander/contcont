import { createContext, useContext } from "react";

import { User } from "./HttpTypes";

export const UserContext = createContext<User>(undefined!);
export const useUser = (): User => useContext(UserContext);
