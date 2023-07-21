import { FormEvent, useEffect, useState } from "react";
import { Helmet } from "react-helmet";
import { useNavigate } from "react-router-dom";

import { usePostArtMutation } from "./MutationHooks";
import { useContestName, useSubmissionOpened } from "./QueryHooks";
import SubmissionNotOpenedView from "./SubmissionNotOpenedView";

export default function ArtSubmitView() {
  const navigate = useNavigate();

  const { data: contestName } = useContestName();
  const { data: opened, isLoading: isOpenedLoading } = useSubmissionOpened();

  const [error, setError] = useState("");

  const { mutate: postArt, isLoading: isPosting } = usePostArtMutation({
    onSuccess: (resp) => {
      navigate(`/art/${resp.id}`);
    },
    onError: (error) => {
      setError((error.response?.data as string) ?? error.message);
    },
  });

  const [title, setTitle] = useState("");
  const [file, setFile] = useState<File | undefined>(undefined);
  const [preview, setPreview] = useState<string | undefined>(undefined);

  useEffect(() => {
    if (!file) {
      setPreview(undefined);
      return;
    }

    const previewUrl = URL.createObjectURL(file);
    setPreview(previewUrl);

    return () => URL.revokeObjectURL(previewUrl);
  }, [file]);

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

  const isInvalid = title === "" || title.length > 100 || file == null;

  const onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (isInvalid) {
      return;
    }

    postArt({ title, file });
  };

  return (
    <>
      <Helmet>
        <title>그림 제출 - {contestName}</title>
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
            <input
              type="file"
              accept="image/png"
              className="file-input file-input-bordered w-full"
              onChange={(e) => {
                setFile(e.target.files?.[0]);
              }}
            />
          </div>
          <input
            type="submit"
            className="btn btn-primary mb-2"
            value="제출"
            disabled={isPosting || isInvalid}
          />
          {file && preview && (
            <div>
              <img className="shadow-lg" src={preview} alt={title} />
            </div>
          )}
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
