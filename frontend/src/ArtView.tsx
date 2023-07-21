import { useState } from "react";
import { Helmet } from "react-helmet";
import { Link, useParams } from "react-router-dom";

import { usePostArtVoteMutation } from "./MutationHooks";
import {
  useArtMetadata,
  useArtVote,
  useContestName,
  useUserFromApi,
  useVotingOpened,
} from "./QueryHooks";

export default function ArtView() {
  const { id } = useParams();
  const { data: contestName } = useContestName();
  const { data: art, isLoading } = useArtMetadata(Number(id));
  const { data: voteOpened } = useVotingOpened();
  const { data: user } = useUserFromApi();
  const { data: vote, refetch: refetchVote } = useArtVote(user, Number(id));

  const [success, setSuccess] = useState("");
  const [error, setError] = useState("");

  const { mutate: postVote, isLoading: isVoting } = usePostArtVoteMutation({
    onSuccess: async () => {
      await refetchVote();
      setSuccess("투표했습니다.");
    },
    onError: (error) => {
      setError((error.response?.data as string) ?? error.message);
    },
  });

  if (isLoading || art == null) {
    return <span className="loading loading-spinner loading-lg" />;
  }

  const onVote = () => {
    postVote({ id: Number(id) });
  };

  return (
    <>
      <Helmet>
        <title>
          {art.title} - {art.authorHandle}@{art.authorInstance} - {contestName}
        </title>
      </Helmet>
      <div className="flex w-screen justify-center">
        <div className="w-2/3 p-4">
          <h2 className="mb-4 text-xl">{art.title}</h2>
          <h2 className="mb-4">
            <Link to={`https://${art.authorInstance}/@${art.authorHandle}`}>
              {art.authorHandle}@{art.authorInstance}
            </Link>
          </h2>
          <img
            src={`/api/contest/art/${art.id}`}
            alt={art.title}
            className="mb-4"
          />
          {voteOpened?.opened ? (
            user != null ? (
              <div>
                <button
                  className="btn btn-primary mr-4"
                  disabled={
                    isVoting ||
                    vote?.voted ||
                    (art.authorHandle === user.handle &&
                      art.authorInstance === user.instance)
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
