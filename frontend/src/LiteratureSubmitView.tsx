import { FormEvent, useState } from "react";
import { Helmet } from "react-helmet";
import { useNavigate } from "react-router-dom";

import { usePostLiteratureMutation } from "./MutationHooks";
import { useContestName, useSubmissionOpened } from "./QueryHooks";
import SubmissionNotOpenedView from "./SubmissionNotOpenedView";

export default function LiteratureSubmitView() {
  const navigate = useNavigate();

  const { data: contestName } = useContestName();
  const { data: opened, isLoading: isOpenedLoading } = useSubmissionOpened();

  const [error, setError] = useState("");

  const { mutate: postLiterature, isLoading: isPosting } =
    usePostLiteratureMutation({
      onSuccess: (resp) => {
        navigate(`/literature/${resp.id}`);
      },
      onError: (error) => {
        setError((error.response?.data as string) ?? error.message);
      },
    });

  const [title, setTitle] = useState("");
  const [text, setText] = useState("");

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

  const isInvalid =
    title === "" || text === "" || title.length > 100 || text.length > 7000;

  const onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (isInvalid) {
      return;
    }

    postLiterature({ title, text });
  };

  return (
    <>
      <Helmet>
        <title>글 제출 - {contestName}</title>
      </Helmet>
      <div className="flex w-screen justify-center">
        <form className="w-2/3" onSubmit={onSubmit}>
          <div className="mb-2">
            <label className="label">
              <label className="label-text">제목</label>
              <label className="label-text-alt">{title.length} / 100</label>
            </label>
            <input
              type="text"
              className="input input-bordered w-full"
              value={title}
              onChange={(e) => {
                setTitle(e.target.value);
              }}
            />
          </div>
          <div className="mb-2">
            <label className="label">
              <label className="label-text">내용</label>
              <label className="label-text-alt">{text.length} / 7000</label>
            </label>
            <textarea
              className="textarea textarea-bordered h-[500px] w-full"
              value={text}
              onChange={(e) => {
                setText(e.target.value);
              }}
            />
          </div>
          <input
            type="submit"
            className="btn btn-primary"
            value="제출"
            disabled={isPosting || isInvalid}
          />
        </form>
      </div>
      {error !== "" && (
        <div className="toast">
          <div className="alert alert-error">
            <span>{error}</span>
          </div>
        </div>
      )}
    </>
  );
}
