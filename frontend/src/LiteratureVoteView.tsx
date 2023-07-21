import { useVotingOpened } from "./QueryHooks";
import VotingNotOpenedView from "./VotingNotOpenedView";

export default function LiteratureVoteView() {
  const { data: opened, isLoading: isOpenedLoading } = useVotingOpened();

  if (isOpenedLoading || opened == null) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  if (!opened.opened) {
    return (
      <VotingNotOpenedView openAt={opened.openAt} closeAt={opened.closeAt} />
    );
  }

  return <></>;
}
