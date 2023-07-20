import { Outlet } from "react-router-dom";
import { useContestName } from "./QueryHooks";

export default function NavBar() {
  const { data: contestName } = useContestName();
  
  return (
    <>
      <div className="navbar bg-base-200">
        <div className="navbar-start p-2">
          <h1 className="text-xl">{contestName}</h1>
        </div>
        <div className="navbar-end p-2">
          <div className="dropdown dropdown-end mr-2">
            <label tabIndex={0} className="btn">글</label>
            <ul tabIndex={0} className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
              <li><a>출품</a></li>
              <li><a>투표</a></li>
            </ul>
          </div>
          <div className="dropdown dropdown-end">
            <label tabIndex={0} className="btn">그림</label>
            <ul tabIndex={0} className="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52">
              <li><a>출품</a></li>
              <li><a>투표</a></li>
            </ul>
          </div>
        </div>
      </div>
      <Outlet />
    </>
  );
}
