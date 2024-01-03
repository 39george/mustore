import { Outlet } from "react-router-dom";
import Menu from "../Menu/Menu";
import { FC } from "react";

const MainLayout: FC = () => {
  return (
    <>
      <Menu />
      <Outlet />
    </>
  );
};

export default MainLayout;
