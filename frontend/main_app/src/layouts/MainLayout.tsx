import { Outlet } from "react-router-dom";
import Menu from "../Menu/Menu";
import { FC } from "react";
import Footer from "../Components/Footer";

const MainLayout: FC = () => {
  return (
    <>
      <Menu />
      <Outlet />
      <Footer />
    </>
  );
};

export default MainLayout;
