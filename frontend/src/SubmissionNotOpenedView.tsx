import { Helmet } from "react-helmet";

import { useContestName } from "./QueryHooks";

interface Props {
  openAt: string;
  closeAt: string;
}

export default function SubmissionNotOpenedView(props: Props) {
  const { data: contestName } = useContestName();

  const openAt = new Date(props.openAt);
  const closeAt = new Date(props.closeAt);

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-full flex-col items-center p-4">
        <h2 className="mb-2 w-fit text-xl">현재 제출이 불가능합니다.</h2>
        <div className="w-fit">
          제출 가능 시간: {openAt.toLocaleString()} ~ {closeAt.toLocaleString()}
        </div>
      </div>
    </>
  );
}
