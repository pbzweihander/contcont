import { FormEvent, useEffect, useState } from "react";
import { Helmet } from "react-helmet";
import { useNavigate } from "react-router-dom";

import LoadingView from "./LoadingView";
import { usePostArtMutation } from "./MutationHooks";
import NotEnabledView from "./NotEnabledView";
import { useContestName, useEnabled, useSubmissionOpened } from "./QueryHooks";
import SubmissionNotOpenedView from "./SubmissionNotOpenedView";

export default function ArtSubmitView() {
  const navigate = useNavigate();

  const { data: contestName } = useContestName();
  const { data: enabled, isLoading: isEnabledLoading } = useEnabled();
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
  const [isNsfw, setIsNsfw] = useState(false);
  const [description, setDescription] = useState("");
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

  if (isEnabledLoading || enabled == null) {
    return <LoadingView />;
  }

  if (!enabled.art) {
    return <NotEnabledView />;
  }

  if (isOpenedLoading || opened == null) {
    return <LoadingView />;
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
    title === "" ||
    title.length > 100 ||
    description === "" ||
    description.length > 2000 ||
    file == null;

  const onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (isInvalid) {
      return;
    }

    if (
      window.confirm(
        "제출합니다.\n제출 후에는 취소하거나 수정할 수 없습니다.\n제출하시겠습니까?"
      )
    ) {
      postArt({ title, description, isNsfw, file });
    }
  };

  return (
    <>
      <Helmet>
        <title>그림 제출 - {contestName}</title>
      </Helmet>
      <div className="flex w-full justify-center px-6 pb-10 pt-4">
        <form className="w-full md:w-2/3" onSubmit={onSubmit}>
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
              <label className="label-text" />
              <label className="label-text-alt">
                PNG 파일만 업로드할 수 있어요.
              </label>
            </label>
            <input
              type="file"
              accept="image/png"
              className="file-input file-input-bordered w-full"
              onChange={(e) => {
                setFile(e.target.files?.[0]);
              }}
            />
          </div>
          <div className="mb-2">
            <label className="label w-fit cursor-pointer">
              <input
                type="checkbox"
                className="checkbox"
                checked={isNsfw}
                onChange={(e) => {
                  setIsNsfw(e.target.checked);
                }}
              />
              <span className="label-text ml-2">NSFW</span>
            </label>
          </div>
          <div className="mb-2">
            <label className="label">
              <label className="label-text">설명</label>
              <label className="label-text-alt">
                {description.length} / 2000
              </label>
            </label>
            <textarea
              className="textarea textarea-bordered h-[200px] w-full"
              value={description}
              onChange={(e) => {
                setDescription(e.target.value);
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
