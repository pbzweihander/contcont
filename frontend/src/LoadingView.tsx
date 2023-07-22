import { Helmet } from "react-helmet";

import { useContestName } from "./QueryHooks";

export default function LoadingView() {
  const { data: contestName } = useContestName();

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-screen justify-center p-10">
        <span className="loading loading-spinner loading-lg" />
      </div>
    </>
  );
}
