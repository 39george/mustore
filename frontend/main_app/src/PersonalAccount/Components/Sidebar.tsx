import styles from "./Sidebar.module.scss";
import { FC } from "react";
import { IoChevronBackOutline } from "react-icons/io5";
import { FaStar } from "react-icons/fa6";
import { NavLink } from "react-router-dom";
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

const Sidebar: FC<SidebarProps> = ({ avatar }) => {
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
          <p className={styles.username}>Alena NAI</p>
          <p className={styles.user_role}>Автор</p>
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
          className={styles.tab_link}
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
