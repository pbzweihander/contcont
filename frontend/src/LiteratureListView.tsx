import { Helmet } from "react-helmet";
import { Link } from "react-router-dom";

import LoadingView from "./LoadingView";
import NotEnabledView from "./NotEnabledView";
import {
  useContestName,
  useEnabled,
  useLiteratureMetadatas,
} from "./QueryHooks";

export default function LiteratureView() {
  const { data: contestName } = useContestName();
  const { data: enabled, isLoading: isEnabledLoading } = useEnabled();
  const { data: literatures, isLoading } = useLiteratureMetadatas();

  if (isEnabledLoading || enabled == null) {
    return <LoadingView />;
  }

  if (!enabled.literature) {
    return <NotEnabledView />;
  }

  if (isLoading || literatures == null) {
    return <LoadingView />;
  }

  return (
    <>
      <Helmet>
        <title>글 - {contestName}</title>
      </Helmet>
      <div className="flex w-full justify-center px-6 pb-10 pt-4">
        <ul className="w-full md:w-2/3">
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
