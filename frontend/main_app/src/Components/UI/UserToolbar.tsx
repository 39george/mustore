import styles from "./UserToolbar.module.scss";
import { FaRegHeart } from "react-icons/fa";
import { PiShoppingCartFill } from "react-icons/pi";
import avatar from "../../assets/HomePage/author_1.png";
import { FC, useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../../state/store";
import {
  ActiveSection,
  select_active_section,
} from "../../state/active_section_slice";

const UserToolbar: FC = () => {
  const [white_color, set_white_color] = useState("");
  const intersecting_section = useSelector<RootState, ActiveSection>((state) =>
    select_active_section(state.active_section)
  );

  useEffect(() => {
    switch (intersecting_section) {
      case "hero":
        set_white_color(``);
        break;
      case "why_us":
        set_white_color(`${styles.white_icons}`);
        break;
      case "group":
        set_white_color(``);
        break;
      case "authors_reviews":
        set_white_color(`${styles.white_icons}`);
        break;
      case "footer":
        set_white_color(``);
        break;
    }
  }, [intersecting_section]);

  return (
    <div className={styles.toolbar}>
      <div className={styles.likes_container}>
        <FaRegHeart className={`${styles.like_icon} ${white_color}`} />
        <div className={styles.likes_amount}>2</div>
      </div>
      <PiShoppingCartFill className={`${styles.cart_icon} ${white_color}`} />
      <div className={styles.user_avatar_container}>
        <img
          src={avatar}
          alt="user's avatar"
          className={styles.user_avatar}
        />
      </div>
    </div>
  );
};

export default UserToolbar;
