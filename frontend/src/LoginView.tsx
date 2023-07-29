import { FormEvent, useState } from "react";
import { Helmet } from "react-helmet";

import { useOauthAuthorizeMutation } from "./MutationHooks";
import { useContestName } from "./QueryHooks";

export default function LoginView() {
  const { data: contestName } = useContestName();

  const [instance, setInstance] = useState("");

  const { mutate: authorizeOauth, isLoading } = useOauthAuthorizeMutation({
    onSuccess: (resp) => {
      window.location.href = resp.url;
    },
  });

  const onSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (instance === "") {
      return;
    }

    authorizeOauth({ instance });
  };

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-full justify-center px-6 pb-10 pt-4">
        <div className="w-full md:w-2/3">
          <div className="mb-2">
            사용 중인 Mastodon/Misskey 계정이 있는 인스턴스를 입력해주세요.
          </div>
          <form onSubmit={onSubmit}>
            <div className="mb-2">
              <input
                type="text"
                className="input input-bordered w-full"
                placeholder="twingyeo.kr"
                value={instance}
                onChange={(e) => {
                  setInstance(e.target.value);
                }}
                disabled={isLoading}
              />
            </div>
            <div>
              <input
                type="submit"
                value="로그인"
                className="btn btn-primary"
                disabled={isLoading}
              />
            </div>
          </form>
        </div>
      </div>
    </>
  );
}
