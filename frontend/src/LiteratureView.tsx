import { useState } from "react";
import { Helmet } from "react-helmet";
import { Link, useParams } from "react-router-dom";

import { usePostLiteratureVoteMutation } from "./MutationHooks";
import {
  useContestName,
  useLiterature,
  useLiteratureVote,
  useUserFromApi,
  useVotingOpened,
} from "./QueryHooks";

export default function LiteratureView() {
  const { id } = useParams();
  const { data: contestName } = useContestName();
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
      onSuccess: () => {
        refetchVote();
        setSuccess("투표했습니다.");
      },
      onError: (error) => {
        setError((error.response?.data as string) ?? error.message);
      },
    });

  if (isLoading || literature == null) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  const onVote = () => {
    postVote({ id: Number(id) });
  };

  return (
    <>
      <Helmet>
        <title>
          {literature.title} - {literature.authorHandle}@
          {literature.authorInstance} - {contestName}
        </title>
      </Helmet>
      <div className="flex w-screen justify-center">
        <div className="w-2/3 p-4">
          <h2 className="mb-4 text-xl">{literature.title}</h2>
          <h2 className="mb-4">
            <Link
              to={`https://${literature.authorInstance}/@${literature.authorHandle}`}
            >
              {literature.authorHandle}@{literature.authorInstance}
            </Link>
          </h2>
          <div className="mb-4 whitespace-pre-line">{literature.text}</div>
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
