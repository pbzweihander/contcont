import { Helmet } from "react-helmet";
import { useContestName } from "./QueryHooks";

export default function MainView() {
  const { data: contestName } = useContestName();

  return <>
    <Helmet>
      <title>{contestName}</title>
    </Helmet>
    <div className="flex w-screen justify-center p-10">
      <ul className="menu w-56">
        <li className="menu-title"><h2 className="text-xl">글</h2></li>
        <li><a className="text-lg">출품</a></li>
        <li><a className="text-lg">투표</a></li>
      </ul>
      <ul className="menu w-56">
        <li className="menu-title"><h2 className="text-xl">그림</h2></li>
        <li><a className="text-lg">출품</a></li>
        <li><a className="text-lg">투표</a></li>
      </ul>
    </div>
  </>;
}
