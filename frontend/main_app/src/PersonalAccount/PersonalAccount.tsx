import styles from "./PersonalAccount.module.scss";
import { Outlet } from "react-router-dom";
import { FC } from "react";
import Sidebar from "./Components/Sidebar";
import TopBar from "./Components/TopBar";

const PersonalAccount: FC = () => {
  return (
    <>
      <Sidebar />
      <>
        <TopBar />
        <Outlet />
      </>
    </>
  );
};

export default PersonalAccount;
