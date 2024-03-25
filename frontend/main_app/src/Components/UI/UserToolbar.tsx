import styles from "./UserToolbar.module.scss";
import { FaRegHeart } from "react-icons/fa";
// import { PiShoppingCartFill } from "react-icons/pi";
import { FC, useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../../state/store";
import {
  ActiveSection,
  select_active_section,
} from "../../state/active_section_slice";
import ToolbarPopUpMenu from "./ToolbarPopUpMenu";
import { LocationNavbar } from "../../types/types";

interface UserToolbarProps {
  location: LocationNavbar;
}

const UserToolbar: FC<UserToolbarProps> = ({ location }) => {
  const [light_colors, set_light_colors] = useState("");
  const intersecting_section = useSelector<RootState, ActiveSection>((state) =>
    select_active_section(state.active_section)
  );
  const [popup_visible, set_popup_visible] = useState(false);
  const [styles_user_avatar, set_styles_user_avatar] = useState(
    `${styles.wrapper}`
  );
  const user_avatar_container_ref = useRef<HTMLDivElement>(null);
  const avatar = useSelector(
    (state: RootState) => state.username_avatar.avatar
  );

  useEffect(() => {
    switch (intersecting_section) {
      case "hero":
        set_light_colors(``);
        break;
      case "why_us":
        set_light_colors(`${styles.light_colors}`);
        break;
      case "group":
        set_light_colors(``);
        break;
      case "authors_reviews":
        set_light_colors(`${styles.light_colors}`);
        break;
      case "footer":
        set_light_colors(``);
        break;
      case null:
        set_light_colors(``);
        break;
    }
  }, [intersecting_section]);

  useEffect(() => {
    if (popup_visible) {
      set_styles_user_avatar(
        `${styles.wrapper} ${styles.user_avatar_container_visible}`
      );
    } else {
      set_styles_user_avatar(`${styles.wrapper}`);
    }
  }, [popup_visible]);

  return (
    <div className={styles.toolbar}>
      <div className={`${styles.wrapper} ${light_colors}`}>
        <div className={styles.likes_container}>
          <FaRegHeart className={styles.like_icon} />
          <div className={styles.likes_amount}>2</div>
        </div>
      </div>
      {/* <div className={`${styles.wrapper} ${light_colors}`}>
        <PiShoppingCartFill className={styles.cart_icon} />
      </div> */}
      <div
        ref={user_avatar_container_ref}
        className={`${styles_user_avatar} ${light_colors}`}
        onClick={() => set_popup_visible(!popup_visible)}
      >
        <div className={styles.user_avatar_container}>
          <img
            src={avatar}
            alt="user's avatar"
            className={styles.user_avatar}
          />
        </div>
        <ToolbarPopUpMenu
          visible={popup_visible}
          set_visible={set_popup_visible}
          user_avatar_container_ref={user_avatar_container_ref.current}
          location={location}
        />
      </div>
    </div>
  );
};

export default UserToolbar;
