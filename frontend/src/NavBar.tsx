import { Link, Outlet, useNavigate } from "react-router-dom";

import { useContestName, useEnabled, useUserFromApi } from "./QueryHooks";

export default function NavBar() {
  const navigate = useNavigate();

  const { data: contestName } = useContestName();
  const { data: enabled } = useEnabled();
  const { data: user, remove: removeUser } = useUserFromApi();

  const onLogout = () => {
    document.cookie = "SESSION=; Max-Age=-99999999;";
    removeUser();
    navigate("/");
  };

  return (
    <>
      <div className="navbar bg-base-200">
        <div className="navbar-start p-2">
          <Link to="/" className="text-xl">
            {contestName}
          </Link>
        </div>
        <div className="navbar-end p-2">
          {enabled?.literature && (
            <div className="dropdown dropdown-end mr-2 hidden md:block">
              <label tabIndex={0} className="btn">
                글
              </label>
              <ul
                tabIndex={0}
                className="menu dropdown-content rounded-box z-[1] w-52 bg-base-100 p-2 shadow"
              >
                <li>
                  <Link to="/literature/submit">출품</Link>
                </li>
                <li>
                  <Link to="/literature">감상 / 투표</Link>
                </li>
                <li>
                  <Link to="/literature/result">결과 확인</Link>
                </li>
              </ul>
            </div>
          )}
          {enabled?.art && (
            <div className="dropdown-end dropdown mr-2 hidden md:block">
              <label tabIndex={0} className="btn break-keep">
                그림
              </label>
              <ul
                tabIndex={0}
                className="menu dropdown-content rounded-box z-[1] w-52 bg-base-100 p-2 shadow"
              >
                <li>
                  <Link to="/art/submit">출품</Link>
                </li>
                <li>
                  <Link to="/art">감상 / 투표</Link>
                </li>
                <li>
                  <Link to="/art/result">결과 확인</Link>
                </li>
              </ul>
            </div>
          )}
          {user != null ? (
            <>
              <span className="mr-2">
                {user.handle}@{user.instance}
              </span>
              <button className="btn" onClick={onLogout}>
                로그아웃
              </button>
            </>
          ) : (
            <Link to="/login" className="btn">
              로그인
            </Link>
          )}
        </div>
      </div>
      <Outlet />
    </>
  );
}
