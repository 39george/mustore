import styles from "./TopBar.module.scss";
import { NavLink, useLocation } from "react-router-dom";
import { FC, useEffect, useState } from "react";
import { FaRegBell } from "react-icons/fa6";
import conversations from "../../assets/icons/conversations_outline.svg";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../../state/store";
import { set_product_status } from "../../state/product_status_slice";
import { FiPlus } from "react-icons/fi";
import { GoChevronDown } from "react-icons/go";

interface TopBarProps {
  username: string;
  avatar: string;
}

const TopBar: FC<TopBarProps> = ({ username, avatar }) => {
  const location = useLocation();
  const dispatch = useDispatch();
  const [header_name, set_header_name] = useState("");
  const current_pathname = location.pathname.replace("/personal-account/", "");
  const product_status = useSelector(
    (state: RootState) => state.product_status.product_status
  );
  const [translated_product_status, set_translated_product_status] =
    useState("");

  useEffect(() => {
    switch (current_pathname) {
      case "dashboard":
        set_header_name("Главная");
        break;
      case "products":
        set_header_name("Товары");
        dispatch(set_product_status("active"));
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

  // Translate product status
  useEffect(() => {
    if (product_status) {
      switch (product_status) {
        case "active":
          set_translated_product_status("аткуальные");
          break;
        case "denied":
          set_translated_product_status("отклоненные");
          break;
        case "hidden":
          set_translated_product_status("скрытые");
          break;
        case "moderation":
          set_translated_product_status("на модерации");
          break;
        case "sold":
          set_translated_product_status("проданные");
          break;
      }
    }
  }, [product_status]);

  return (
    <div className={styles.top_bar}>
      <div className={styles.header_and_widgets}>
        <h2 className={styles.h2}>{header_name}</h2>
        {product_status && (
          <div className={styles.products_widgets}>
            <div className={styles.product_status}>
              <p className={styles.product_status_p}>
                {translated_product_status} <span>(5)</span>
              </p>
              <GoChevronDown className={styles.chevron} />
            </div>
            <div className={styles.upload_product}>
              <p className={styles.upload_product_p}>загрузить новый товар</p>
              <FiPlus className={styles.plus_icon} />
            </div>
            <div className={styles.actions_button}>
              <p className={styles.actions_text}>действия</p>
              <GoChevronDown className={styles.chevron} />
            </div>
          </div>
        )}
      </div>
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
            <NavLink
              to="/"
              onClick={() => dispatch(set_product_status(null))}
            >
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
