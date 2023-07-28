import { Helmet } from "react-helmet";

import { useContestName } from "./QueryHooks";

export default function NotFoundView() {
  const { data: contestName } = useContestName();

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-screen flex-col items-center p-4">
        <h2 className="mb-2 w-fit text-xl">찾을 수 없는 페이지입니다.</h2>
      </div>
    </>
  );
}
