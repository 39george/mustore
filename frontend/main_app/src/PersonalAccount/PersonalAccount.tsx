import styles from "./PersonalAccount.module.scss";
import { Outlet } from "react-router-dom";
import { FC } from "react";
import Sidebar from "./Components/Sidebar";
import TopBar from "./Components/TopBar";
import avatar from "../assets/HomePage/author_1.png";

const PersonalAccount: FC = () => {
  return (
    <div className={styles.personal_account}>
      <Sidebar avatar={avatar} />
      <div className={styles.main_view}>
        <TopBar avatar={avatar} />
        <hr className={styles.top_border} />
        <Outlet />
      </div>
    </div>
  );
};

export default PersonalAccount;
