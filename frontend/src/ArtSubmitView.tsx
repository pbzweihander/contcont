import { useSubmissionOpened } from "./QueryHooks";
import SubmissionNotOpenedView from "./SubmissionNotOpenedView";

export default function ArtSubmitView() {
  const { data: opened, isLoading: isOpenedLoading } = useSubmissionOpened();

  if (isOpenedLoading || opened == null) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  if (!opened.opened) {
    return (
      <SubmissionNotOpenedView
        openAt={opened.openAt}
        closeAt={opened.closeAt}
      />
    );
  }

  return <></>;
}
