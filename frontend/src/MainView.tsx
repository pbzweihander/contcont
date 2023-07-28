import { Helmet } from "react-helmet";
import { Link } from "react-router-dom";

import { useContestName, useEnabled } from "./QueryHooks";

export default function MainView() {
  const { data: contestName } = useContestName();
  const { data: enabled } = useEnabled();

  return (
    <>
      <Helmet>
        <title>{contestName}</title>
      </Helmet>
      <div className="flex w-full justify-center p-10">
        {enabled?.literature && (
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
        )}
        {enabled?.art && (
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
        )}
      </div>
    </>
  );
}
