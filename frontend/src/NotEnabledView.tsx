import { Helmet } from "react-helmet";

import { useContestName } from "./QueryHooks";

export default function NotEnabledView() {
  const { data: contestName } = useContestName();

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-screen flex-col items-center p-4">
        <h2 className="mb-2 w-fit text-xl">활성화되지 않은 기능입니다.</h2>
      </div>
    </>
  );
}
