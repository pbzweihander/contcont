import { Link, Outlet } from "react-router-dom";

import { useContestName, useUserFromApi } from "./QueryHooks";

export default function NavBar() {
  const { data: contestName } = useContestName();
  const { data: user } = useUserFromApi();

  return (
    <>
      <div className="navbar bg-base-200">
        <div className="navbar-start p-2">
          <Link to="/" className="text-xl">
            {contestName}
          </Link>
        </div>
        <div className="navbar-end p-2">
          <div className="dropdown dropdown-end mr-2">
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
                <Link to="/literature/vote">투표</Link>
              </li>
            </ul>
          </div>
          <div className="dropdown-end dropdown mr-2">
            <label tabIndex={0} className="btn">
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
                <Link to="/art/vote">투표</Link>
              </li>
            </ul>
          </div>
          {user != null ? (
            <span>
              {user.handle}@{user.instance}
            </span>
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
