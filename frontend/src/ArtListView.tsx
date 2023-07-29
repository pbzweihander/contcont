import classNames from "classnames";
import { Helmet } from "react-helmet";
import { Link } from "react-router-dom";

import LoadingView from "./LoadingView";
import NotEnabledView from "./NotEnabledView";
import { useArtMetadatas, useContestName, useEnabled } from "./QueryHooks";

export default function ArtListView() {
  const { data: contestName } = useContestName();
  const { data: enabled, isLoading: isEnabledLoading } = useEnabled();
  const { data: arts, isLoading } = useArtMetadatas();

  if (isEnabledLoading || enabled == null) {
    return <LoadingView />;
  }

  if (!enabled.art) {
    return <NotEnabledView />;
  }

  if (isLoading || arts == null) {
    return <LoadingView />;
  }

  return (
    <>
      <Helmet>
        <title>그림 - {contestName}</title>
      </Helmet>
      <div className="flex w-full flex-wrap gap-10 p-6">
        {arts.map((art) => (
          <Link key={art.id} to={`/art/${art.id}`} className="w-full md:w-96">
            <div className="card shadow-xl">
              <figure>
                <img
                  src={`/api/contest/art/thumbnail/${art.id}`}
                  className={classNames("h-[200px]", art.isNsfw && "blur-lg")}
                />
              </figure>
              <div className="card-body">
                <h2 className="card-title">
                  {art.isNsfw && (
                    <span className="badge badge-secondary mr-2">NSFW</span>
                  )}
                  {art.title}
                </h2>
                <span>
                  {art.authorHandle}@{art.authorInstance}
                </span>
              </div>
            </div>
          </Link>
        ))}
      </div>
    </>
  );
}
