import { QueryClient, QueryClientProvider } from "react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { createClient } from "./Axios";
import { AxiosClientProvider } from "./AxiosContext";
import MainView from "./MainView";
import NavBar from "./NavBar";

const queryClient = new QueryClient();
const axiosClient = createClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <AxiosClientProvider value={axiosClient}>
        <BrowserRouter>
          <Routes>
            <Route element={<NavBar />}>
              <Route path="/" element={<MainView />} />
            </Route>
          </Routes>
        </BrowserRouter>
      </AxiosClientProvider>
    </QueryClientProvider>
  );
}

export default App;
