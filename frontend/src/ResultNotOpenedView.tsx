import { Helmet } from "react-helmet";

import { useContestName } from "./QueryHooks";

interface Props {
  openAt: string;
}

export default function ResultNotOpenedView(props: Props) {
  const { data: contestName } = useContestName();

  const openAt = new Date(props.openAt);

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-full flex-col items-center p-4">
        <h2 className="mb-2 w-fit text-xl">현재 결과 확인이 불가능합니다.</h2>
        <div className="w-fit">
          결과 확인 가능 시간: {openAt.toLocaleString()} 부터
        </div>
      </div>
    </>
  );
}
