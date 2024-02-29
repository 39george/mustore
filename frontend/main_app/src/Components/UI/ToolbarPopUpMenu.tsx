import styles from "./ToolbarPopUpMenu.module.scss";
import { FC, useEffect, useRef } from "react";
import useLogOutUserApi from "../../hooks/API/useLogOutUserApi";
import useCheckPermissionsApi from "../../hooks/API/useCheckPermissionsApi";
import { NavLink } from "react-router-dom";
import { useSelector } from "react-redux";
import { RootState } from "../../state/store";

interface ToolbarPopUpMenuProps {
  visible: boolean;
  set_visible: (visible: boolean) => void;
  user_avatar_container_ref: HTMLDivElement | null;
}

const ToolbarPopUpMenu: FC<ToolbarPopUpMenuProps> = ({
  visible,
  set_visible,
  user_avatar_container_ref,
}) => {
  const toolbar_popup_menu_ref = useRef<HTMLDivElement>(null);
  const { logout } = useLogOutUserApi();
  const { check_user_permissions } = useCheckPermissionsApi();
  const username = useSelector(
    (state: RootState) => state.username_avatar.username
  );
  const avatar = useSelector(
    (state: RootState) => state.username_avatar.avatar
  );

  console.log(avatar);

  useEffect(() => {
    const handle_click_outside_popup = (e: MouseEvent) => {
      if (user_avatar_container_ref?.contains(e.target as Node)) {
        return;
      }

      if (
        toolbar_popup_menu_ref.current &&
        !toolbar_popup_menu_ref.current.contains(e.target as Node)
      ) {
        set_visible(false);
      }
    };

    document.addEventListener("mousedown", handle_click_outside_popup);

    return () => {
      document.removeEventListener("mousedown", handle_click_outside_popup);
    };
  }, [user_avatar_container_ref]);

  const try_to_logout = async () => {
    await logout();
    await check_user_permissions();
  };

  return (
    <div
      ref={toolbar_popup_menu_ref}
      className={styles.toolbar_popup}
      style={{
        opacity: `${visible ? "1" : "0"}`,
        visibility: `${visible ? "visible" : "hidden"}`,
      }}
      onClick={(e) => e.stopPropagation()}
    >
      <div className={styles.metainfo}>
        <div className={styles.wrapper}>
          <div className={styles.user_avatar_container}>
            <img
              src={avatar}
              alt="user's avatar"
              className={styles.user_avatar}
            />
          </div>
        </div>
        <p className={styles.username}>{username}</p>
      </div>
      <hr className={styles.divider} />
      <ul className={styles.options}>
        <li className={styles.option_account}>
          <NavLink
            to="personal-account/dashboard"
            className={styles.option}
          >
            Личный кабинет
          </NavLink>
        </li>
        <li
          className={styles.option}
          onClick={try_to_logout}
        >
          Выйти
        </li>
      </ul>
    </div>
  );
};

export default ToolbarPopUpMenu;
