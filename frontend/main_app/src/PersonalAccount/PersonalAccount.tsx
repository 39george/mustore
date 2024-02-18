import styles from "./PersonalAccount.module.scss";
import { Outlet } from "react-router-dom";
import { FC } from "react";
import Sidebar from "./Components/Sidebar";
import TopBar from "./Components/TopBar";

const PersonalAccount: FC = () => {
  return (
    <div className={styles.personal_account}>
      <Sidebar />
      <div className={styles.main_view}>
        <TopBar />
        <Outlet />
      </div>
    </div>
  );
};

export default PersonalAccount;
