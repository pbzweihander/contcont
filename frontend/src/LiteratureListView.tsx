import { Helmet } from "react-helmet";
import { Link } from "react-router-dom";

import LoadingView from "./LoadingView";
import { useContestName, useLiteratureMetadatas } from "./QueryHooks";

export default function LiteratureView() {
  const { data: contestName } = useContestName();
  const { data: literatures, isLoading } = useLiteratureMetadatas();

  if (isLoading || literatures == null) {
    return <LoadingView />;
  }

  return (
    <>
      <Helmet>
        <title>글 - {contestName}</title>
      </Helmet>
      <div className="flex w-screen justify-center">
        <ul className="w-2/3 p-4">
          {literatures.map((literature) => (
            <li key={literature.id} className="p-2">
              <Link to={`/literature/${literature.id}`}>
                {literature.isNsfw && (
                  <span className="badge badge-secondary mr-2">NSFW</span>
                )}
                {literature.authorHandle}@{literature.authorInstance} -{" "}
                {literature.title}
              </Link>
            </li>
          ))}
        </ul>
      </div>
    </>
  );
}
