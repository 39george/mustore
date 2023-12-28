import styles from "./Menu.module.scss";
import { Link, NavLink } from "react-router-dom";
import { GoChevronDown } from "react-icons/go";
import { FaYoutube, FaVk, FaTelegram } from "react-icons/fa6";
import { BsInstagram } from "react-icons/bs";
import { IoMenu } from "react-icons/io5";
import { HiMiniXMark } from "react-icons/hi2";
import { FC, useEffect, useRef, useState } from "react";
import logo from "../assets/svg/logo.svg";
import { LinkName, ToggledLinks } from "../types/types";

const Menu: FC = () => {
  const [link_toggled, set_link_toggled] = useState<ToggledLinks>({
    products: false,
    services: false,
    help: false,
    about: false,
  });
  const [sidebar_open, set_sidebar_open] = useState(false);
  const sidebar_ref = useRef<HTMLDivElement>(null);

  const toggle_link = (link_name: LinkName) => {
    set_link_toggled((prev_state) => ({
      ...prev_state,
      [link_name]: !prev_state[link_name],
    }));
  };

  const toggle_sidebar = () => {
    set_sidebar_open(!sidebar_open);
  };

  useEffect(() => {
    function handle_click_outside(event: MouseEvent) {
      if (
        sidebar_ref.current &&
        !sidebar_ref.current.contains(event.target as Node)
      ) {
        set_sidebar_open(false);
      }
    }

    function handle_scroll_outside(event: Event) {
      if (
        sidebar_ref.current &&
        !sidebar_ref.current.contains(event.target as Node)
      ) {
        set_sidebar_open(false);
      }
    }

    document.addEventListener("mousedown", handle_click_outside);
    document.addEventListener("scroll", handle_scroll_outside);

    return () => {
      document.removeEventListener("mousedown", handle_click_outside);
      document.removeEventListener("scroll", handle_scroll_outside);
    };
  }, []);

  return (
    <nav className={styles.nav_bar}>
      <ul className={styles.nav_links}>
        <li className={styles.logo}>
          <NavLink
            to="."
            end
          >
            <img
              src={logo}
              alt="logo"
            />
          </NavLink>
        </li>
        <li className={styles.link_container}>
          <div className={styles.nav_link}>
            Товары
            <GoChevronDown className={styles.chevron} />
          </div>
          <ul className={styles.submenu}>
            <li className={styles.submenu_header}>Товары</li>
            <div className={styles.submenu_body}>
              <li>
                <NavLink to="products">Песни</NavLink>
              </li>
              <li>
                <NavLink to="products">Биты</NavLink>
              </li>
              <li>
                <NavLink to="products">Обложки</NavLink>
              </li>
              <li>
                <NavLink to="products">Тексты</NavLink>
              </li>
            </div>
          </ul>
        </li>
        <li className={styles.link_container}>
          <div className={styles.nav_link}>
            Услуги
            <GoChevronDown className={styles.chevron} />
          </div>
          <ul className={styles.submenu}>
            <li className={styles.submenu_header}>Услуги</li>
            <div className={styles.submenu_body}>
              <li>
                <NavLink to="services">Создание песни</NavLink>
              </li>
              <li>
                <NavLink to="services">Сведение / Мастеринг</NavLink>
              </li>
              <li>
                <NavLink to="services">Написание бита</NavLink>
              </li>
              <li>
                <NavLink to="services">Гострайтинг</NavLink>
              </li>
              <li>
                <NavLink to="services">Дизайн обложки</NavLink>
              </li>
            </div>
          </ul>
        </li>
        <li className={styles.link_container}>
          <div className={styles.nav_link}>
            Помощь
            <GoChevronDown className={styles.chevron} />
          </div>
          <ul className={styles.submenu}>
            <li className={styles.submenu_header}>Помощь</li>
            <div className={styles.submenu_body}>
              <li>
                <NavLink to="help">FAQ</NavLink>
              </li>
              <li>
                <NavLink to="help">Обратиться в поддержку</NavLink>
              </li>
              <li>
                <NavLink to="help">Покупателям</NavLink>
              </li>
              <li>
                <NavLink to="help">Продавцам</NavLink>
              </li>
              <li>
                <NavLink to="help">Политика конфиденциальности</NavLink>
              </li>
              <li>
                <NavLink to="help">Правила сообщества</NavLink>
              </li>
            </div>
          </ul>
        </li>
        <li className={styles.link_container}>
          <div className={styles.nav_link}>
            О нас
            <GoChevronDown className={styles.chevron} />
          </div>
          <ul className={styles.submenu}>
            <li className={styles.submenu_header}>О нас</li>
            <div className={styles.submenu_body}>
              <li>
                <NavLink to="about">Блог</NavLink>
              </li>
              <li>
                <NavLink to="about">Наша команда</NavLink>
              </li>
            </div>
          </ul>
        </li>
      </ul>
      <div className={styles.logging}>
        <div className={styles.log}>войти</div>
        <div className={styles.divider}>|</div>
        <div className={styles.log}>создать аккаунт</div>
      </div>
      <div className={styles.toggle_icons_container}>
        <IoMenu
          className={`${styles.burger_icon} ${sidebar_open ? "" : styles.show}`}
          onClick={toggle_sidebar}
        />
        <HiMiniXMark
          className={`${styles.close_icon} ${sidebar_open ? "" : styles.hide}`}
          onClick={toggle_sidebar}
        />
      </div>
      {
        // Sidebar
      }
      <div
        className={`${styles.mobile_sidebar} ${
          sidebar_open ? styles.sidebar_visible : ""
        }`}
        ref={sidebar_ref}
      >
        <div className={styles.sidebar_content}>
          <div className={styles.sidebar_nav_links}>
            <li
              className={
                link_toggled.products
                  ? `${styles.sidebar_link_container} ${styles.link_toggled}`
                  : styles.sidebar_link_container
              }
            >
              <div
                className={styles.sidebar_nav_link}
                onClick={() => toggle_link("products")}
              >
                <div>Товары</div>
                <GoChevronDown className={styles.sidebar_chevron} />
              </div>
              <ul className={styles.sidebar_submenu}>
                <li>
                  <NavLink to="products">Песни</NavLink>
                </li>
                <li>
                  <NavLink to="products">Биты</NavLink>
                </li>
                <li>
                  <NavLink to="products">Обложки</NavLink>
                </li>
                <li>
                  <NavLink to="products">Тексты</NavLink>
                </li>
              </ul>
            </li>
            <li
              className={
                link_toggled.services
                  ? `${styles.sidebar_link_container} ${styles.link_toggled}`
                  : styles.sidebar_link_container
              }
            >
              <div
                className={styles.sidebar_nav_link}
                onClick={() => toggle_link("services")}
              >
                <div>Услуги</div>
                <GoChevronDown className={styles.sidebar_chevron} />
              </div>
              <ul className={styles.sidebar_submenu}>
                <li>
                  <NavLink to="services">Создание песни</NavLink>
                </li>
                <li>
                  <NavLink to="services">Сведение / Мастеринг</NavLink>
                </li>
                <li>
                  <NavLink to="services">Написание бита</NavLink>
                </li>
                <li>
                  <NavLink to="services">Гострайтинг</NavLink>
                </li>
                <li>
                  <NavLink to="services">Дизайн обложки</NavLink>
                </li>
              </ul>
            </li>
            <li
              className={
                link_toggled.help
                  ? `${styles.sidebar_link_container} ${styles.link_toggled}`
                  : styles.sidebar_link_container
              }
            >
              <div
                className={styles.sidebar_nav_link}
                onClick={() => toggle_link("help")}
              >
                <div>Помощь</div>
                <GoChevronDown className={styles.sidebar_chevron} />
              </div>
              <ul className={styles.sidebar_submenu}>
                <li>
                  <NavLink to="help">FAQ</NavLink>
                </li>
                <li>
                  <NavLink to="help">Обратиться в поддержку</NavLink>
                </li>
                <li>
                  <NavLink to="help">Покупателям</NavLink>
                </li>
                <li>
                  <NavLink to="help">Продавцам</NavLink>
                </li>
                <li>
                  <NavLink to="help">Политика конфиденциальности</NavLink>
                </li>
                <li>
                  <NavLink to="help">Правила сообщества</NavLink>
                </li>
              </ul>
            </li>
            <li
              className={
                link_toggled.about
                  ? `${styles.sidebar_link_container} ${styles.link_toggled}`
                  : styles.sidebar_link_container
              }
            >
              <div
                className={styles.sidebar_nav_link}
                onClick={() => toggle_link("about")}
              >
                <div>О нас</div>
                <GoChevronDown className={styles.sidebar_chevron} />
              </div>
              <ul className={styles.sidebar_submenu}>
                <li>
                  <NavLink to="about">Блог</NavLink>
                </li>
                <li>
                  <NavLink to="about">Наша команда</NavLink>
                </li>
              </ul>
            </li>
          </div>
          <hr />
          <div className={styles.sidebar_logging}>
            <div className={styles.sidebar_log}>войти</div>
            <div className={styles.sidebar_log}>создать аккаунт</div>
          </div>
        </div>
        <div className={styles.sidebar_footer}>
          <div className={styles.copyright}>
            ©2024 Mustore, all rights reserved
          </div>
          <div className={styles.social_media_container}>
            <Link
              to="https://www.youtube.com/"
              target="_blank"
            >
              <FaYoutube className={styles.social_media} />
            </Link>
            <Link
              to="https://vk.com/"
              target="_blank"
            >
              <FaVk className={styles.social_media} />
            </Link>
            <Link
              to="https://web.telegram.org/"
              target="_blank"
            >
              <FaTelegram className={styles.social_media} />
            </Link>
            <Link
              to="https://www.instagram.com/"
              target="_blank"
            >
              <BsInstagram className={styles.social_media} />
            </Link>
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Menu;
