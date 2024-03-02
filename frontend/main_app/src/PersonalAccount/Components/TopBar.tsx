import styles from "./TopBar.module.scss";
import { NavLink, useLocation } from "react-router-dom";
import { FC, useEffect, useState } from "react";
import { FaRegBell } from "react-icons/fa6";
import conversations from "../../assets/icons/conversations_outline.svg";

interface TopBarProps {
  username: string;
  avatar: string;
}

const TopBar: FC<TopBarProps> = ({ username, avatar }) => {
  const [header_name, set_header_name] = useState("");
  const location = useLocation();
  const current_pathname = location.pathname.replace("/personal-account/", "");

  useEffect(() => {
    switch (current_pathname) {
      case "dashboard":
        set_header_name("Главная");
        break;
      case "products":
        set_header_name("Товары");
        break;
      case "services":
        set_header_name("Услуги");
        break;
      case "conversations":
        set_header_name("Беседы");
        break;
      case "orders":
        set_header_name("Заказы");
        break;
      case "statistics":
        set_header_name("Статистика");
        break;
      case "earnings":
        set_header_name("Заработок");
        break;
      case "settings":
        set_header_name("Настройки");
        break;
      case "notifications":
        set_header_name("Уведомления");
        break;
      case "help":
        set_header_name("Помощь");
        break;
      default:
        set_header_name("Страница не найдена");
    }
  }, [current_pathname]);

  return (
    <div className={styles.top_bar}>
      <h2 className={styles.h2}>{header_name}</h2>
      <div className={styles.interactions_container}>
        <div className={styles.notifications}>
          <FaRegBell className={styles.notifications_icon} />
          <span className={styles.notifications_dot}></span>
        </div>
        <div className={styles.conversations}>
          <img
            src={conversations}
            alt="conversations icon"
          />
          <span className={styles.notifications_dot}></span>
        </div>
        <hr className={styles.divider} />
        <div className={styles.meta_info_container}>
          <p className={styles.username}>{username}</p>
          <div className={styles.avatar_container}>
            <NavLink to="/">
              <div className={styles.image_wrapper}>
                <img
                  src={avatar}
                  alt="users's avatar"
                  className={styles.avatar}
                />
              </div>
            </NavLink>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TopBar;
