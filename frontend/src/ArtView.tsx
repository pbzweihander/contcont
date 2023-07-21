import { Helmet } from "react-helmet";
import { Link, useParams } from "react-router-dom";

import { useArtMetadata, useContestName } from "./QueryHooks";

export default function ArtView() {
  const { id } = useParams();
  const { data: contestName } = useContestName();
  const { data: art, isLoading } = useArtMetadata(Number(id));

  if (isLoading || art == null) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  return (
    <>
      <Helmet>
        <title>
          {art.title} - {art.authorHandle}@{art.authorInstance} - {contestName}
        </title>
      </Helmet>
      <div className="flex w-screen justify-center">
        <div className="w-2/3 p-4">
          <h2 className="mb-4 text-xl">{art.title}</h2>
          <h2 className="mb-4">
            <Link to={`https://${art.authorInstance}/@${art.authorHandle}`}>
              {art.authorHandle}@{art.authorInstance}
            </Link>
          </h2>
          <img src={`/api/contest/art/${art.id}`} alt={art.title} />
        </div>
      </div>
    </>
  );
}
