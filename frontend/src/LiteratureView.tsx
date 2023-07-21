import { Helmet } from "react-helmet";
import { useParams } from "react-router-dom";

import { useContestName, useLiterature } from "./QueryHooks";

export default function LiteratureView() {
  const { id } = useParams();
  const { data: contestName } = useContestName();
  const { data: literature, isLoading } = useLiterature(Number(id));

  if (isLoading || literature == null) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  return (
    <>
      <Helmet>
        <title>
          {literature.title} - {contestName}
        </title>
      </Helmet>
      <div className="flex w-screen justify-center">
        <div className="w-2/3 p-4">
          <h2 className="mb-4 text-xl">{literature.title}</h2>
          <div className="whitespace-pre-line">{literature.text}</div>
        </div>
      </div>
    </>
  );
}
