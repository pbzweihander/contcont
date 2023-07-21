import { QueryClient, QueryClientProvider } from "react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import ArtListView from "./ArtListView";
import ArtSubmitView from "./ArtSubmitView";
import ArtView from "./ArtView";
import { AuthRequired } from "./Auth";
import { createClient } from "./Axios";
import { AxiosClientProvider } from "./AxiosContext";
import LiteratureListView from "./LiteratureListView";
import LiteratureSubmitView from "./LiteratureSubmitView";
import LiteratureView from "./LiteratureView";
import LoginView from "./LoginView";
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

              <Route path="/login" element={<LoginView />} />

              <Route path="/literature" element={<LiteratureListView />} />
              <Route
                path="/literature/submit"
                element={
                  <AuthRequired>
                    <LiteratureSubmitView />
                  </AuthRequired>
                }
              />
              <Route path="/literature/:id" element={<LiteratureView />} />

              <Route path="/art" element={<ArtListView />} />
              <Route
                path="/art/submit"
                element={
                  <AuthRequired>
                    <ArtSubmitView />
                  </AuthRequired>
                }
              />
              <Route path="/art/:id" element={<ArtView />} />
            </Route>
          </Routes>
        </BrowserRouter>
      </AxiosClientProvider>
    </QueryClientProvider>
  );
}

export default App;
