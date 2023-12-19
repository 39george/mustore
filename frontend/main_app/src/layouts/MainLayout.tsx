import { Outlet } from "react-router-dom";
import Menu from "../Menu/Menu";

const MainLayout = () => {
  return (
    <>
      <Menu />
      <Outlet />
    </>
  );
};

export default MainLayout;
