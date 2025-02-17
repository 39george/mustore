import styles from "./Sidebar.module.scss";
import React, { FC, useEffect, useState } from "react";
import chevron from "../../assets/icons/chevron.svg";
import { FaStar } from "react-icons/fa6";
import { NavLink, useLocation } from "react-router-dom";
import dashboard_icon from "../../assets/icons/dashboard.svg";
import products_icon from "../../assets/icons/products.svg";
import services_icon from "../../assets/icons/services.svg";
import conversations_icon from "../../assets/icons/conversations.svg";
import orders_icon from "../../assets/icons/orders.svg";
import statistics_icon from "../../assets/icons/statistics.svg";
import earnings_icon from "../../assets/icons/earnings.svg";
import settings_icon from "../../assets/icons/settings.svg";
import notifications_icon from "../../assets/icons/notifications.svg";
import help_icon from "../../assets/icons/help.svg";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../../state/store";
import {
  set_sidebar_chevron_display,
  set_sidebar_collapsed,
  set_sidebar_title,
} from "../../state/sidebar_actions_slice";
import { ActiveTabsAccountCreator } from "../../types/types";
import { set_product_status } from "../../state/product_status_slice";
import { set_active_tab_account_creator } from "../../state/active_tab_account_creator_slice";

const class_fade_in = `${styles.class_fade_in}`;
const class_fade_in_image = `${styles.class_fade_in_image}`;

interface SidebarProps {
  username: string;
  user_role: string;
  avatar: string;
}

const Sidebar: FC<SidebarProps> = ({ username, user_role, avatar }) => {
  const location = useLocation();
  const current_pathname = location.pathname.replace("/personal-account/", "");
  const dispatch = useDispatch();
  const sidebar_collapsed = useSelector(
    (state: RootState) => state.sidebar_actions.sidebar_collapsed
  );
  const title = useSelector(
    (state: RootState) => state.sidebar_actions.sidebar_title
  );
  const active_tab = useSelector(
    (state: RootState) => state.active_tab_account_creator.active_tab
  );
  const chevron_display_style = useSelector(
    (state: RootState) => state.sidebar_actions.sidebar_chevron_display
  );
  const title_parts = title.split(".");
  const [after_rolldown, set_after_rolldown] = useState(``);
  const [chevron_disabled, set_chevron_disabled] = useState(
    current_pathname === "conversations" ? `${styles.chevron_disabled}` : ``
  );

  useEffect(() => {
    const handle_resize = () => {
      if (window.innerWidth <= 950) {
        dispatch(set_sidebar_collapsed(true));
        dispatch(set_sidebar_title("H.S"));
        dispatch(set_sidebar_chevron_display("none"));
      } else {
        dispatch(set_sidebar_chevron_display("block"));
      }
    };

    window.addEventListener("resize", handle_resize);

    return () => {
      window.removeEventListener("resize", handle_resize);
    };
  }, []);

  const handle_tab_link_click = (tab_name: ActiveTabsAccountCreator) => {
    dispatch(set_active_tab_account_creator(tab_name));
  };

  useEffect(() => {
    // Disable functionality of sidebar collapsed icon if path's "conversations"
    if (active_tab === "conversations") {
      dispatch(set_sidebar_collapsed(true));
      dispatch(set_sidebar_title("H.S"));

      if (window.innerWidth <= 950) {
        dispatch(set_sidebar_chevron_display("none"));
      } else {
        set_chevron_disabled(`${styles.chevron_disabled}`);
      }
    } else {
      set_chevron_disabled(``);
      if (window.innerWidth > 950) {
        dispatch(set_sidebar_chevron_display("block"));
      }
    }

    // Custom settings for path "products"
    if (active_tab !== "products") {
      dispatch(set_product_status(null));
    }
  }, [active_tab]);

  const handle_collapse_icon_click = () => {
    if (chevron_disabled) {
      return;
    }

    dispatch(set_sidebar_collapsed(!sidebar_collapsed));
  };

  useEffect(() => {
    if (!sidebar_collapsed) {
      dispatch(set_sidebar_title("HARMONY.SPHERE"));
      setTimeout(() => {
        set_after_rolldown(`${styles.after_rolldown}`);
      }, 400);
    } else {
      dispatch(set_sidebar_title("H.S"));
      set_after_rolldown(``);
    }
  }, [sidebar_collapsed]);

  return (
    <div
      className={`${styles.sidebar} ${
        sidebar_collapsed && styles.sidebar_collapsed
      }`}
    >
      <div className={styles.filler_sidebar}></div>
      <div
        className={`${styles.collapse_icon_container} ${chevron_disabled}`}
        style={{ display: `${chevron_display_style}` }}
        onClick={handle_collapse_icon_click}
      >
        <img
          src={chevron}
          alt="chveron icon"
          className={styles.collapse_icon}
        />
        <img
          src={chevron}
          alt="chveron icon"
          className={styles.collapse_icon}
        />
      </div>
      <h2 className={`${styles.h2} ${!sidebar_collapsed && class_fade_in}`}>
        {title_parts.map((part, idx) => (
          <React.Fragment key={idx}>
            {part}
            {idx < title_parts.length - 1 && <span>.</span>}
          </React.Fragment>
        ))}
      </h2>
      <div className={styles.meta_info_container}>
        <div
          className={`${styles.image_wrapper} ${
            !sidebar_collapsed && class_fade_in_image
          }`}
        >
          <img
            src={avatar}
            alt="user's avatar"
          />
        </div>
        <div
          className={`${styles.meta_text} ${
            !sidebar_collapsed && class_fade_in
          }`}
        >
          <p className={styles.username}>{username}</p>
          <p className={styles.user_role}>{user_role}</p>
          <div className={styles.rating_container}>
            <FaStar className={styles.star_icon} />
            <p className={styles.rating}>
              5<span>(23)</span>
            </p>
          </div>
        </div>
      </div>
      <div
        className={`${styles.tabs_container} ${
          !sidebar_collapsed && after_rolldown
        } `}
      >
        <NavLink
          to="dashboard"
          className={`${styles.tab_link} ${styles.dashboard} ${
            active_tab === "dashboard" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("dashboard")}
        >
          <img
            src={dashboard_icon}
            alt="dashboard_icon"
            className={styles.tabs_icon}
          />
          <p>Главная</p>
        </NavLink>
        <p className={`${styles.section_name} ${styles.section_content}`}>
          Контент
        </p>
        <NavLink
          to="products"
          className={`${styles.tab_link} ${styles.products} ${
            active_tab === "products" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("products")}
        >
          <img
            src={products_icon}
            alt="products_icon"
            className={styles.tabs_icon}
          />
          <p>Товары</p>
        </NavLink>
        <NavLink
          to="services"
          className={`${styles.tab_link} ${styles.services} ${
            active_tab === "services" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("services")}
        >
          <img
            src={services_icon}
            alt="services_icon"
            className={styles.tabs_icon}
          />
          <p>Услуги</p>
        </NavLink>
        <p className={`${styles.section_name} ${styles.section_coop}`}>
          Сотрудничесвто
        </p>
        <NavLink
          to="conversations"
          className={`${styles.tab_link} ${styles.conversations} ${
            active_tab === "conversations" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("conversations")}
        >
          <img
            src={conversations_icon}
            alt="conversations_icon"
            className={styles.tabs_icon}
          />
          <p>Беседы</p>
        </NavLink>
        <NavLink
          to="orders"
          className={`${styles.tab_link} ${styles.orders} ${
            active_tab === "orders" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("orders")}
        >
          <img
            src={orders_icon}
            alt="orders_icon"
            className={styles.tabs_icon}
          />
          <p>Заказы</p>
        </NavLink>
        <p className={`${styles.section_name} ${styles.section_efficiency}`}>
          Эффективность
        </p>
        <NavLink
          to="statistics"
          className={`${styles.tab_link} ${styles.statistics} ${
            active_tab === "statistics" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("statistics")}
        >
          <img
            src={statistics_icon}
            alt="statistics_icon"
            className={styles.tabs_icon}
          />
          <p>Статистика</p>
        </NavLink>
        <NavLink
          to="earnings"
          className={`${styles.tab_link} ${styles.earnings} ${
            active_tab === "earnings" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("earnings")}
        >
          <img
            src={earnings_icon}
            alt="earnings_icon"
            className={styles.tabs_icon}
          />
          <p>Заработок</p>
        </NavLink>
        <p className={`${styles.section_name} ${styles.section_account}`}>
          Аккаунт
        </p>
        <NavLink
          to="settings"
          className={`${styles.tab_link} ${styles.settings} ${
            active_tab === "settings" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("settings")}
        >
          <img
            src={settings_icon}
            alt="settings_icon"
            className={styles.tabs_icon}
          />
          <p>Настройки</p>
        </NavLink>
        <p className={`${styles.section_name} ${styles.section_general}`}>
          Общее
        </p>
        <NavLink
          to="notifications"
          className={`${styles.tab_link} ${styles.notifications} ${
            active_tab === "notifications" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("notifications")}
        >
          <img
            src={notifications_icon}
            alt="notifications_icon"
            className={styles.tabs_icon}
          />
          <p>Уведомления</p>
        </NavLink>
        <NavLink
          to="help"
          className={`${styles.tab_link} ${styles.help} ${
            active_tab === "help" && styles.tab_link_active
          }`}
          onClick={() => handle_tab_link_click("help")}
        >
          <img
            src={help_icon}
            alt="help_icon"
            className={styles.tabs_icon}
          />
          <p>Помощь</p>
        </NavLink>
      </div>
    </div>
  );
};

export default Sidebar;
