import { useState } from "react";
import { Helmet } from "react-helmet";
import { Link, useParams } from "react-router-dom";

import LoadingView from "./LoadingView";
import { usePostLiteratureVoteMutation } from "./MutationHooks";
import NotEnabledView from "./NotEnabledView";
import NotFoundView from "./NotFoundView";
import {
  useContestName,
  useEnabled,
  useLiterature,
  useLiteratureVote,
  useUserFromApi,
  useVotingOpened,
} from "./QueryHooks";

export default function LiteratureView() {
  const { id } = useParams();
  const { data: contestName } = useContestName();
  const { data: enabled, isLoading: isEnabledLoading } = useEnabled();
  const { data: literature, isLoading } = useLiterature(Number(id));
  const { data: voteOpened } = useVotingOpened();
  const { data: user } = useUserFromApi();
  const { data: vote, refetch: refetchVote } = useLiteratureVote(
    user,
    Number(id)
  );

  const [success, setSuccess] = useState("");
  const [error, setError] = useState("");

  const { mutate: postVote, isLoading: isVoting } =
    usePostLiteratureVoteMutation({
      onSuccess: async () => {
        await refetchVote();
        setSuccess("투표했습니다.");
      },
      onError: (error) => {
        setError((error.response?.data as string) ?? error.message);
      },
    });

  if (isEnabledLoading || enabled == null) {
    return <LoadingView />;
  }

  if (!enabled.literature) {
    return <NotEnabledView />;
  }

  if (isLoading) {
    return <LoadingView />;
  }

  if (literature == null) {
    return <NotFoundView />;
  }

  const onVote = () => {
    if (!voteOpened?.opened) {
      return;
    }

    if (
      window.confirm(
        "투표합니다.\n투표 후에는 취소할 수 없습니다.\n투표하시겠습니까?"
      )
    ) {
      postVote({ id: Number(id) });
    }
  };

  return (
    <>
      <Helmet>
        <title>
          {literature.title} - {literature.authorHandle}@
          {literature.authorInstance} - {contestName}
        </title>
      </Helmet>
      <div className="flex w-full justify-center">
        <div className="w-2/3 p-4">
          <h2 className="mb-4 text-xl">
            {literature.isNsfw && (
              <span className="badge badge-secondary mr-2">NSFW</span>
            )}
            {literature.title}
          </h2>
          <h3 className="mb-4">
            <Link
              to={`https://${literature.authorInstance}/@${literature.authorHandle}`}
            >
              {literature.authorHandle}@{literature.authorInstance}
            </Link>
          </h3>
          <p className="mb-4 whitespace-pre-line">{literature.text}</p>
          <div className="divider" />
          {voteOpened?.opened ? (
            user != null ? (
              <div>
                <button
                  className="btn btn-primary mr-4"
                  disabled={
                    isVoting ||
                    vote?.voted ||
                    (literature.authorHandle === user.handle &&
                      literature.authorInstance === user.instance)
                  }
                  onClick={onVote}
                >
                  투표하기
                </button>
                <span>현재 자신의 투표 수: {vote?.voteCount} / 5</span>
              </div>
            ) : (
              <div>
                투표는{" "}
                <Link to="/login" className="text-blue-600 underline">
                  로그인
                </Link>{" "}
                후에 가능합니다.
              </div>
            )
          ) : (
            <div>
              현재 투표가 불가능합니다. 투표 가능 시간:{" "}
              {new Date(voteOpened?.openAt ?? 0).toLocaleString()} ~{" "}
              {new Date(voteOpened?.closeAt ?? 0).toLocaleString()}
            </div>
          )}
        </div>
      </div>
      {(success !== "" || error !== "") && (
        <div className="toast">
          {success !== "" && (
            <div className="alert alert-success">
              <span>{success}</span>
            </div>
          )}
          {error !== "" && (
            <div className="alert alert-error">
              <span>{error}</span>
            </div>
          )}
        </div>
      )}
    </>
  );
}
