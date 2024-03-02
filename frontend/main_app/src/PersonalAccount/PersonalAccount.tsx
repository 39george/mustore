import styles from "./PersonalAccount.module.scss";
import { Outlet } from "react-router-dom";
import { FC, useEffect, useState } from "react";
import Sidebar from "./Components/Sidebar";
import TopBar from "./Components/TopBar";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";

const PersonalAccount: FC = () => {
  const username = useSelector(
    (state: RootState) => state.username_avatar.username
  );
  const avatar = useSelector(
    (state: RootState) => state.username_avatar.avatar
  );
  const user_permissions = useSelector(
    (state: RootState) => state.user_permissions
  );
  const [user_role, set_user_role] = useState("...");

  useEffect(() => {
    const index_creator = user_permissions.permissions.findIndex(
      (obj) => obj.name === "creator"
    );
    if (user_permissions.permissions[index_creator]?.name === "creator") {
      set_user_role("Автор");
    }
  }, [user_permissions]);

  return (
    <div className={styles.personal_account}>
      <Sidebar
        username={username}
        user_role={user_role}
        avatar={avatar}
      />
      <div className={styles.main_view}>
        <TopBar
          username={username}
          avatar={avatar}
        />
        <hr className={styles.top_border} />
        <Outlet />
      </div>
    </div>
  );
};

export default PersonalAccount;
