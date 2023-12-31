import classNames from "classnames";
import { useState } from "react";
import { Helmet } from "react-helmet";
import { Link, useParams } from "react-router-dom";

import LoadingView from "./LoadingView";
import { usePostArtVoteMutation } from "./MutationHooks";
import NotEnabledView from "./NotEnabledView";
import NotFoundView from "./NotFoundView";
import {
  useArtMetadata,
  useArtVote,
  useContestName,
  useEnabled,
  useUserFromApi,
  useVotingOpened,
} from "./QueryHooks";

export default function ArtView() {
  const { id } = useParams();
  const { data: contestName } = useContestName();
  const { data: enabled, isLoading: isEnabledLoading } = useEnabled();
  const { data: art, isLoading } = useArtMetadata(Number(id));
  const { data: voteOpened } = useVotingOpened();
  const { data: user } = useUserFromApi();
  const { data: vote, refetch: refetchVote } = useArtVote(user, Number(id));

  const [success, setSuccess] = useState("");
  const [error, setError] = useState("");

  const [isBlurRemoved, setIsBlurRemoved] = useState(false);

  const { mutate: postVote, isLoading: isVoting } = usePostArtVoteMutation({
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

  if (!enabled.art) {
    return <NotEnabledView />;
  }

  if (isLoading) {
    return <LoadingView />;
  }

  if (art == null) {
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
          {art.title} - {art.authorHandle}@{art.authorInstance} - {contestName}
        </title>
      </Helmet>
      <div className="flex w-full justify-center px-6 pb-10 pt-4">
        <div className="w-full md:w-2/3">
          <h2 className="mb-4 text-xl">
            {art.isNsfw && (
              <span className="badge badge-secondary mr-2">NSFW</span>
            )}
            {art.title}
          </h2>
          <h2 className="mb-4">
            <Link to={`https://${art.authorInstance}/@${art.authorHandle}`}>
              {art.authorHandle}@{art.authorInstance}
            </Link>
          </h2>
          <img
            src={`/api/contest/art/${art.id}`}
            alt={art.title}
            className={classNames(
              "mb-4 w-full md:w-fit",
              art.isNsfw && "cursor-pointer",
              art.isNsfw && !isBlurRemoved && "blur-lg"
            )}
            onClick={() => {
              if (art.isNsfw) {
                setIsBlurRemoved((b) => !b);
              }
            }}
          />
          <p className="mb-4 whitespace-pre-line">{art.description}</p>
          <div className="divider" />
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
