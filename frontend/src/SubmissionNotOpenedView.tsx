interface Props {
  openAt: string;
  closeAt: string;
}

export default function SubmissionNotOpenedView(props: Props) {
  const openAt = new Date(props.openAt);
  const closeAt = new Date(props.closeAt);

  return (
    <div className="flex w-screen flex-col items-center p-4">
      <h2 className="mb-2 w-fit text-xl">현재 제출이 불가능합니다.</h2>
      <div className="w-fit">
        제출 가능 시간: {openAt.toLocaleString()} ~ {closeAt.toLocaleString()}
      </div>
    </div>
  );
}
