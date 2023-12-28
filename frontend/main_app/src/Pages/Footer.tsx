import styles from "./Footer.module.scss";
import { Link, NavLink } from "react-router-dom";
import { FC, useState } from "react";
import logo from "../assets/svg/logo.svg";
import { FaYoutube, FaVk, FaTelegram } from "react-icons/fa6";
import { BsInstagram } from "react-icons/bs";
import { GoChevronDown } from "react-icons/go";
import { LinkName, ToggledLinks } from "../types/types";

const Footer: FC = () => {
  const [link_toggled, set_link_toggled] = useState<ToggledLinks>({
    products: false,
    services: false,
    help: false,
    about: false,
  });

  const toggle_link = (link_name: LinkName) => {
    set_link_toggled((prev_state) => ({
      ...prev_state,
      [link_name]: !prev_state[link_name],
    }));
  };

  return (
    <footer className={styles.footer}>
      <hr className={styles.divider} />
      <div className={styles.main_content}>
        <ul className={styles.nav_links}>
          <li
            className={
              link_toggled.products
                ? `${styles.links_container} ${styles.link_toggled}`
                : styles.links_container
            }
          >
            <div
              className={styles.link_header}
              onClick={() => toggle_link("products")}
            >
              <p>Товары</p>
              <GoChevronDown className={styles.chevron} />
            </div>
            <ul className={styles.links}>
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
                ? `${styles.links_container} ${styles.link_toggled}`
                : styles.links_container
            }
          >
            <div
              className={styles.link_header}
              onClick={() => toggle_link("services")}
            >
              <p>Услуги</p>
              <GoChevronDown className={styles.chevron} />
            </div>
            <ul className={styles.links}>
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
                ? `${styles.links_container} ${styles.link_toggled}`
                : styles.links_container
            }
          >
            <div
              className={styles.link_header}
              onClick={() => toggle_link("help")}
            >
              <p>Помощь</p>
              <GoChevronDown className={styles.chevron} />
            </div>
            <ul className={styles.links}>
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
                ? `${styles.links_container} ${styles.link_toggled}`
                : styles.links_container
            }
          >
            <div
              className={styles.link_header}
              onClick={() => toggle_link("about")}
            >
              <p>О нас</p>
              <GoChevronDown className={styles.chevron} />
            </div>
            <ul className={styles.links}>
              <li>
                <NavLink to="about">Блог</NavLink>
              </li>
              <li>
                <NavLink to="about">Наша команда</NavLink>
              </li>
            </ul>
          </li>
        </ul>
        <hr className={styles.divider} />
        <ul className={styles.bottom_bar}>
          <li>
            <NavLink to=".">
              <img
                src={logo}
                alt="logo"
              />
            </NavLink>
          </li>
          <li className={styles.copyright}>
            ©2024 Mustore, all rights reserved.
          </li>
          <li className={styles.social_media_container}>
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
          </li>
        </ul>
      </div>
    </footer>
  );
};

export default Footer;
