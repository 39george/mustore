import styles from "./PersonalAccount.module.scss";
import { Outlet } from "react-router-dom";
import { FC } from "react";
import Sidebar from "./Components/Sidebar";
import TopBar from "./Components/TopBar";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import { find_user_role_index, translate_user_role } from "../helpers/helpers";

const PersonalAccount: FC = () => {
  const username_avatar = useSelector(
    (state: RootState) => state.username_avatar
  );
  const user_permissions = useSelector(
    (state: RootState) => state.user_permissions
  );

  return (
    <div className={styles.personal_account}>
      <Sidebar
        username={username_avatar.is_loading ? "..." : username_avatar.username}
        user_role={
          user_permissions.is_loading
            ? "..."
            : translate_user_role(
                user_permissions.permissions[
                  find_user_role_index(user_permissions.permissions, "creator")
                ].name
              )
        }
        avatar={username_avatar.is_loading ? "..." : username_avatar.avatar}
      />
      <div className={styles.main_view}>
        <TopBar
          username={
            username_avatar.is_loading ? "..." : username_avatar.username
          }
          avatar={username_avatar.is_loading ? "..." : username_avatar.avatar}
        />
        <hr className={styles.top_border} />
        <Outlet />
      </div>
      <div className={styles.filler_main}></div>
    </div>
  );
};

export default PersonalAccount;
