import styles from "./Sidebar.module.scss";
import { FC, useEffect, useState } from "react";
import { IoChevronBackOutline } from "react-icons/io5";
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
import logo_account from "../../assets/icons/logo_account.svg";

interface SidebarProps {
  avatar: string;
}

type ActiveSections =
  | "dashboard"
  | "products"
  | "services"
  | "conversations"
  | "orders"
  | "statistics"
  | "earnings"
  | "settings"
  | "notifications"
  | "help"
  | "none";

const Sidebar: FC<SidebarProps> = ({ avatar }) => {
  const [active_section, set_active_section] = useState<ActiveSections>("none");
  const location = useLocation();
  const current_pathname = location.pathname.replace("/personal-account/", "");

  useEffect(() => {
    switch (current_pathname) {
      case "dashboard":
        set_active_section("dashboard");
        break;
      case "products":
        set_active_section("products");
        break;
      case "services":
        set_active_section("services");
        break;
      case "conversations":
        set_active_section("conversations");
        break;
      case "orders":
        set_active_section("orders");
        break;
      case "statistics":
        set_active_section("statistics");
        break;
      case "earnings":
        set_active_section("earnings");
        break;
      case "settings":
        set_active_section("settings");
        break;
      case "notifications":
        set_active_section("notifications");
        break;
      case "help":
        set_active_section("help");
        break;
      default:
        set_active_section("none");
    }
  }, [current_pathname]);

  return (
    <div className={styles.sidebar}>
      <div className={styles.collapse_icon_container}>
        <IoChevronBackOutline className={styles.collapse_icon} />
        <IoChevronBackOutline className={styles.collapse_icon} />
      </div>
      <h2 className={styles.h2}>
        HARMONY<span>.</span>SPHERE
      </h2>
      <div className={styles.meta_info_container}>
        <div className={styles.image_wrapper}>
          <img
            src={avatar}
            alt="user's avatar"
          />
        </div>
        <div className={styles.meta_text}>
          <p className={styles.user_role}>Автор</p>
          <p className={styles.username}>Alena NAI</p>
          <div className={styles.rating_container}>
            <FaStar className={styles.star_icon} />
            <p className={styles.rating}>
              5<span>(23)</span>
            </p>
          </div>
        </div>
      </div>
      <div className={styles.tabs_container}>
        <NavLink
          to="dashboard"
          className={`${styles.tab_link} ${
            active_section === "dashboard" && styles.tab_link_active
          }`}
        >
          <img
            src={dashboard_icon}
            alt="dashboard_icon"
            className={styles.tabs_icon}
          />
          <p>Главная</p>
        </NavLink>
        <p className={styles.section_name}>Контент</p>
        <NavLink
          to="products"
          className={`${styles.tab_link} ${
            active_section === "products" && styles.tab_link_active
          }`}
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
          className={`${styles.tab_link} ${
            active_section === "services" && styles.tab_link_active
          }`}
        >
          <img
            src={services_icon}
            alt="services_icon"
            className={styles.tabs_icon}
          />
          <p>Услуги</p>
        </NavLink>
        <p className={styles.section_name}>Сотрудничесвто</p>
        <NavLink
          to="conversations"
          className={`${styles.tab_link} ${
            active_section === "conversations" && styles.tab_link_active
          }`}
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
          className={`${styles.tab_link} ${
            active_section === "orders" && styles.tab_link_active
          }`}
        >
          <img
            src={orders_icon}
            alt="orders_icon"
            className={styles.tabs_icon}
          />
          <p>Заказы</p>
        </NavLink>
        <p className={styles.section_name}>Эффективность</p>
        <NavLink
          to="statistics"
          className={`${styles.tab_link} ${
            active_section === "statistics" && styles.tab_link_active
          }`}
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
          className={`${styles.tab_link} ${
            active_section === "earnings" && styles.tab_link_active
          }`}
        >
          <img
            src={earnings_icon}
            alt="earnings_icon"
            className={styles.tabs_icon}
          />
          <p>Заработок</p>
        </NavLink>
        <p className={styles.section_name}>Аккаунт</p>
        <NavLink
          to="settings"
          className={`${styles.tab_link} ${
            active_section === "settings" && styles.tab_link_active
          }`}
        >
          <img
            src={settings_icon}
            alt="settings_icon"
            className={styles.tabs_icon}
          />
          <p>Настройки</p>
        </NavLink>
        <p className={styles.section_name}>Общее</p>
        <NavLink
          to="notifications"
          className={`${styles.tab_link} ${
            active_section === "notifications" && styles.tab_link_active
          }`}
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
          className={`${styles.tab_link} ${
            active_section === "help" && styles.tab_link_active
          }`}
        >
          <img
            src={help_icon}
            alt="help_icon"
            className={styles.tabs_icon}
          />
          <p>Помощь</p>
        </NavLink>
      </div>
      <img
        src={logo_account}
        alt="logo"
        className={styles.logo_icon}
      />
    </div>
  );
};

export default Sidebar;
