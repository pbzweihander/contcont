import { Helmet } from "react-helmet";
import { Link } from "react-router-dom";

import { useContestName } from "./QueryHooks";

export default function MainView() {
  const { data: contestName } = useContestName();

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-screen justify-center p-10">
        <ul className="menu w-56">
          <li className="menu-title">
            <h2 className="text-xl">글</h2>
          </li>
          <li>
            <Link to="/literature/submit" className="text-lg">
              출품
            </Link>
          </li>
          <li>
            <Link to="/literature" className="text-lg">
              감상 / 투표
            </Link>
          </li>
        </ul>
        <ul className="menu w-56">
          <li className="menu-title">
            <h2 className="text-xl">그림</h2>
          </li>
          <li>
            <Link to="/art/submit" className="text-lg">
              출품
            </Link>
          </li>
          <li>
            <Link to="/art" className="text-lg">
              감상 / 투표
            </Link>
          </li>
        </ul>
      </div>
    </>
  );
}
