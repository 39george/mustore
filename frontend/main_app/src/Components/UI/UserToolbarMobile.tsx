import styles from "./UserToolbarMobile.module.scss";
import { FC, useEffect, useState } from "react";
import { GoChevronDown } from "react-icons/go";
import { FaRegHeart } from "react-icons/fa";
import { PiShoppingCartFill } from "react-icons/pi";
import useLogOutUserApi from "../../hooks/API/useLogOutUserApi";
import useCheckPermissionsApi from "../../hooks/API/useCheckPermissionsApi";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../../state/store";
import { NavLink } from "react-router-dom";
import { set_active_tab_account_creator } from "../../state/active_tab_account_creator_slice";

interface UserToolbarMobileProps {
  sidebar_open: boolean;
}

const UserToolbarMobile: FC<UserToolbarMobileProps> = ({ sidebar_open }) => {
  const { logout } = useLogOutUserApi();
  const { check_user_permissions } = useCheckPermissionsApi();
  const [options_visible, set_options_visible] = useState(false);
  const [chevron_styles, set_chevron_styles] = useState(`${styles.chevron}`);
  const username = useSelector(
    (state: RootState) => state.username_avatar.username
  );
  const avatar = useSelector(
    (state: RootState) => state.username_avatar.avatar
  );
  const dispatch = useDispatch();

  const try_to_logout = async () => {
    await logout();
    await check_user_permissions();
  };

  const handle_account_link_click = () => {
    dispatch(set_active_tab_account_creator("dashboard"));
  };

  useEffect(() => {
    if (options_visible) {
      set_chevron_styles(`${styles.chevron} ${styles.chevron_active}`);
    } else {
      set_chevron_styles(`${styles.chevron}`);
    }
  }, [options_visible]);

  useEffect(() => {
    if (!sidebar_open) {
      set_options_visible(false);
    }
  }, [sidebar_open]);

  return (
    <div className={styles.toolbar_mobile}>
      <div
        className={styles.metainfo}
        onClick={() => set_options_visible(!options_visible)}
      >
        <div className={styles.user_avatar_container}>
          <img
            src={avatar}
            alt="user's avatar"
            className={styles.user_avatar}
          />
        </div>
        <p className={styles.username}>{username}</p>
        <GoChevronDown className={chevron_styles} />
      </div>
      <ul
        className={styles.options}
        style={{ display: `${options_visible ? "block" : "none"}` }}
      >
        <li className={styles.option}>
          <NavLink
            to="personal-account/dashboard"
            className={styles.option}
            onClick={handle_account_link_click}
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
      <div className={styles.actions}>
        <div className={styles.likes_container}>
          <FaRegHeart className={styles.like_icon} />
          <div className={styles.likes_amount}>2</div>
        </div>
        <PiShoppingCartFill className={styles.cart_icon} />
      </div>
    </div>
  );
};

export default UserToolbarMobile;
