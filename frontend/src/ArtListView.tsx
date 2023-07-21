import { Helmet } from "react-helmet";
import { Link } from "react-router-dom";

import { useArtMetadatas, useContestName } from "./QueryHooks";

export default function ArtListView() {
  const { data: contestName } = useContestName();
  const { data: arts, isLoading } = useArtMetadatas();

  if (isLoading || arts == null) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  return (
    <>
      <Helmet>
        <title>그림 - {contestName}</title>
      </Helmet>
      <div className="flex w-screen flex-wrap gap-10 p-4">
        {arts.map((art) => (
          <Link key={art.id} to={`/art/${art.id}`} className="w-fit">
            <div className="card w-96 shadow-xl">
              <figure>
                <img src={`/api/contest/art/thumbnail/${art.id}`} />
              </figure>
              <div className="card-body">
                <h2 className="card-title">{art.title}</h2>
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
