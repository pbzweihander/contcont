import { QueryClient, QueryClientProvider } from "react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import ArtListView from "./ArtListView";
import ArtResultView from "./ArtResultView";
import ArtSubmitView from "./ArtSubmitView";
import ArtView from "./ArtView";
import { AuthRequired } from "./AuthRequired";
import { createClient } from "./Axios";
import { AxiosClientProvider } from "./AxiosContext";
import LiteratureListView from "./LiteratureListView";
import LiteratureResultView from "./LiteratureResultView";
import LiteratureSubmitView from "./LiteratureSubmitView";
import LiteratureView from "./LiteratureView";
import LoginView from "./LoginView";
import MainView from "./MainView";
import NavBar from "./NavBar";
import NotFoundView from "./NotFoundView";

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
              <Route
                path="/literature/result"
                element={<LiteratureResultView />}
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
              <Route path="/art/result" element={<ArtResultView />} />
              <Route path="/art/:id" element={<ArtView />} />

              <Route path="*" element={<NotFoundView />} />
            </Route>
          </Routes>
        </BrowserRouter>
      </AxiosClientProvider>
    </QueryClientProvider>
  );
}

export default App;
