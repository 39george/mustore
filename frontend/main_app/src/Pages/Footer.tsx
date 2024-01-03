import styles from "./Footer.module.scss";
import { Link, NavLink } from "react-router-dom";
import { FC, useState } from "react";
import logo from "../assets/svg/logo.svg";
import { FaYoutube, FaVk, FaTelegram } from "react-icons/fa6";
import { BsInstagram } from "react-icons/bs";
import { GoChevronDown } from "react-icons/go";
import { LinkName, ToggledLinks } from "../types/types";
import usePageNavigation from "../hooks/usePageNavigation";

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

  const close_all = () => {
    set_link_toggled({
      products: false,
      services: false,
      help: false,
      about: false,
    });
  };

  const handle_page_navigation = usePageNavigation();

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
                <NavLink
                  to="products/songs"
                  onClick={() => {
                    handle_page_navigation("products/songs");
                    close_all();
                  }}
                >
                  Песни
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="products/beats"
                  onClick={() => {
                    handle_page_navigation("products/beats");
                    close_all();
                  }}
                >
                  Биты
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="products/covers"
                  onClick={() => {
                    handle_page_navigation("products/covers");
                    close_all();
                  }}
                >
                  Обложки
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="products/texts"
                  onClick={() => {
                    handle_page_navigation("products/texts");
                    close_all();
                  }}
                >
                  Тексты
                </NavLink>
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
                <NavLink
                  to="services"
                  onClick={() => {
                    handle_page_navigation("services");
                    close_all();
                  }}
                >
                  Создание песни
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="services"
                  onClick={() => {
                    handle_page_navigation("services");
                    close_all();
                  }}
                >
                  Сведение / Мастеринг
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="services"
                  onClick={() => {
                    handle_page_navigation("services");
                    close_all();
                  }}
                >
                  Написание бита
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="services"
                  onClick={() => {
                    handle_page_navigation("services");
                    close_all();
                  }}
                >
                  Гострайтинг
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="services"
                  onClick={() => {
                    handle_page_navigation("services");
                    close_all();
                  }}
                >
                  Дизайн обложки
                </NavLink>
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
                <NavLink
                  to="help"
                  onClick={() => {
                    handle_page_navigation("help");
                    close_all();
                  }}
                >
                  FAQ
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="help"
                  onClick={() => {
                    handle_page_navigation("help");
                    close_all();
                  }}
                >
                  Обратиться в поддержку
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="help"
                  onClick={() => {
                    handle_page_navigation("help");
                    close_all();
                  }}
                >
                  Покупателям
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="help"
                  onClick={() => {
                    handle_page_navigation("help");
                    close_all();
                  }}
                >
                  Продавцам
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="help"
                  onClick={() => {
                    handle_page_navigation("help");
                    close_all();
                  }}
                >
                  Политика конфиденциальности
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="help"
                  onClick={() => {
                    handle_page_navigation("help");
                    close_all();
                  }}
                >
                  Правила сообщества
                </NavLink>
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
                <NavLink
                  to="about"
                  onClick={() => {
                    handle_page_navigation("about");
                    close_all();
                  }}
                >
                  Блог
                </NavLink>
              </li>
              <li>
                <NavLink
                  to="about"
                  onClick={() => {
                    handle_page_navigation("about");
                    close_all();
                  }}
                >
                  Наша команда
                </NavLink>
              </li>
            </ul>
          </li>
        </ul>
        <hr className={styles.divider} />
        <ul className={styles.bottom_bar}>
          <li>
            <NavLink
              to="."
              onClick={() => handle_page_navigation("")}
            >
              <img
                src={logo}
                alt="logo"
              />
            </NavLink>
          </li>
          <li className={styles.copyright}>
            ©2024 HARMONY.SPHERE, all rights reserved.
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
