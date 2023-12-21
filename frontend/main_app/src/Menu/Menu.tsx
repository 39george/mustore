import { NavLink } from "react-router-dom";
import { GoChevronDown } from "react-icons/go";
import { FaYoutube, FaVk, FaTelegram } from "react-icons/fa6";
import { BsInstagram } from "react-icons/bs";
import { IoMenu } from "react-icons/io5";
import { HiMiniXMark } from "react-icons/hi2";
import styles from "./Menu.module.scss";
import { FC, useState } from "react";

interface ToggledLinks {
  products: boolean;
  services: boolean;
  help: boolean;
  about: boolean;
}

type LinkName = keyof ToggledLinks;

const Menu: FC = () => {
  const [link_toggled, set_link_toggled] = useState<ToggledLinks>({
    products: false,
    services: false,
    help: false,
    about: false,
  });
  const [sidebar_open, set_sidebar_open] = useState(false);

  const toggle_link = (link_name: LinkName) => {
    set_link_toggled((prev_state) => ({
      ...prev_state,
      [link_name]: !prev_state[link_name],
    }));
  };

  const toggle_sidebar = () => {
    set_sidebar_open(!sidebar_open);
  };

  return (
    <nav className={styles.nav_bar}>
      <ul className={styles.nav_links}>
        <li className={styles.logo}>
          <NavLink
            to="."
            end
          >
            <svg
              width="89"
              height="36"
              viewBox="0 0 222 68"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M112 59L98.5 66.7942V51.2058L112 59Z"
                fill="#FF005C"
              />
              <rect
                x="116"
                y="51"
                width="2"
                height="15"
                fill="#FF005C"
              />
              <rect
                x="120"
                y="51"
                width="2"
                height="15"
                fill="#FF005C"
              />
              <rect
                x="45"
                y="57"
                width="48"
                height="3"
                fill="#FF005C"
              />
              <rect
                x="130"
                y="57"
                width="48"
                height="3"
                fill="#FF005C"
              />
              <path
                d="M4.0939 40V12H9.5739L19.7739 28.6L16.1739 28.56L26.4939 12H31.7339V40H25.8539V30.4C25.8539 28 25.9072 25.84 26.0139 23.92C26.1472 22 26.3606 20.0933 26.6539 18.2L27.3739 20.08L19.1739 32.8H16.5339L8.4939 20.16L9.1739 18.2C9.46724 19.9867 9.66724 21.8267 9.7739 23.72C9.90724 25.5867 9.9739 27.8133 9.9739 30.4V40H4.0939ZM54.1358 40.24C51.8691 40.24 49.8424 39.7733 48.0558 38.84C46.2958 37.88 44.9091 36.5733 43.8958 34.92C42.8824 33.24 42.3758 31.3467 42.3758 29.24V11.96H48.4158V28.88C48.4158 30 48.6691 31.0133 49.1758 31.92C49.6824 32.8 50.3624 33.5067 51.2158 34.04C52.0958 34.5733 53.0691 34.84 54.1358 34.84C55.2291 34.84 56.2158 34.5733 57.0958 34.04C58.0024 33.5067 58.7091 32.8 59.2158 31.92C59.7491 31.0133 60.0158 30 60.0158 28.88V11.96H65.8558V29.24C65.8558 31.3467 65.3491 33.24 64.3358 34.92C63.3224 36.5733 61.9224 37.88 60.1358 38.84C58.3758 39.7733 56.3758 40.24 54.1358 40.24ZM85.1786 40.4C83.4453 40.4 81.8586 40.1867 80.4186 39.76C79.0053 39.3333 77.7386 38.7067 76.6186 37.88C75.5253 37.0533 74.5386 36.0533 73.6586 34.88L77.4186 30.6C78.7253 32.4133 80.0186 33.6533 81.2986 34.32C82.6053 34.96 84.0186 35.28 85.5386 35.28C86.3653 35.28 87.0986 35.1733 87.7386 34.96C88.4053 34.72 88.9253 34.3867 89.2986 33.96C89.6719 33.5333 89.8586 33.0267 89.8586 32.44C89.8586 32.0133 89.7653 31.6267 89.5786 31.28C89.3919 30.9067 89.1253 30.5867 88.7786 30.32C88.4319 30.0267 88.0186 29.7733 87.5386 29.56C87.0586 29.32 86.5253 29.12 85.9386 28.96C85.3519 28.7733 84.7119 28.6267 84.0186 28.52C82.5253 28.1733 81.2186 27.76 80.0986 27.28C78.9786 26.7733 78.0453 26.16 77.2986 25.44C76.5519 24.6933 75.9919 23.8533 75.6186 22.92C75.2719 21.9867 75.0986 20.9333 75.0986 19.76C75.0986 18.56 75.3653 17.4533 75.8986 16.44C76.4319 15.4 77.1653 14.5067 78.0986 13.76C79.0586 13.0133 80.1519 12.44 81.3786 12.04C82.6319 11.64 83.9519 11.44 85.3386 11.44C87.0453 11.44 88.5386 11.64 89.8186 12.04C91.0986 12.4133 92.2053 12.96 93.1386 13.68C94.0986 14.4 94.8986 15.28 95.5386 16.32L91.7386 20C91.1786 19.2533 90.5653 18.6267 89.8986 18.12C89.2586 17.6133 88.5519 17.24 87.7786 17C87.0319 16.7333 86.2586 16.6 85.4586 16.6C84.5786 16.6 83.8186 16.72 83.1786 16.96C82.5386 17.1733 82.0319 17.4933 81.6586 17.92C81.3119 18.3467 81.1386 18.8667 81.1386 19.48C81.1386 19.96 81.2586 20.3867 81.4986 20.76C81.7386 21.1333 82.0853 21.4667 82.5386 21.76C82.9919 22.0267 83.5386 22.2667 84.1786 22.48C84.8186 22.6933 85.5253 22.88 86.2986 23.04C87.7653 23.3333 89.0853 23.72 90.2586 24.2C91.4319 24.68 92.4319 25.2667 93.2586 25.96C94.1119 26.6267 94.7653 27.4267 95.2186 28.36C95.6719 29.2667 95.8986 30.2933 95.8986 31.44C95.8986 33.3333 95.4453 34.9467 94.5386 36.28C93.6319 37.6133 92.3786 38.64 90.7786 39.36C89.1786 40.0533 87.3119 40.4 85.1786 40.4ZM110.773 40V17.4H102.613V12H125.173V17.4H116.773V40H110.773ZM144.917 40.4C142.864 40.4 140.971 40.04 139.237 39.32C137.504 38.6 135.984 37.6 134.677 36.32C133.397 35.0133 132.411 33.48 131.717 31.72C131.024 29.9333 130.677 28 130.677 25.92C130.677 23.8133 131.024 21.8933 131.717 20.16C132.411 18.4 133.397 16.8667 134.677 15.56C135.984 14.2533 137.504 13.24 139.237 12.52C140.971 11.8 142.864 11.44 144.917 11.44C146.997 11.44 148.891 11.8 150.597 12.52C152.331 13.24 153.837 14.2533 155.117 15.56C156.424 16.8667 157.424 18.4 158.117 20.16C158.811 21.92 159.157 23.84 159.157 25.92C159.157 28 158.811 29.92 158.117 31.68C157.424 33.44 156.424 34.9733 155.117 36.28C153.837 37.5867 152.331 38.6 150.597 39.32C148.891 40.04 146.997 40.4 144.917 40.4ZM144.917 34.72C146.091 34.72 147.171 34.5067 148.157 34.08C149.171 33.6533 150.037 33.04 150.757 32.24C151.477 31.44 152.037 30.5067 152.437 29.44C152.864 28.3733 153.077 27.2 153.077 25.92C153.077 24.64 152.864 23.4667 152.437 22.4C152.037 21.3333 151.477 20.4133 150.757 19.64C150.037 18.84 149.171 18.2267 148.157 17.8C147.171 17.3467 146.091 17.12 144.917 17.12C143.744 17.12 142.664 17.3467 141.677 17.8C140.691 18.2267 139.824 18.84 139.077 19.64C138.331 20.4133 137.757 21.3333 137.357 22.4C136.984 23.4667 136.797 24.64 136.797 25.92C136.797 27.2 136.984 28.3733 137.357 29.44C137.757 30.5067 138.331 31.44 139.077 32.24C139.824 33.04 140.691 33.6533 141.677 34.08C142.664 34.5067 143.744 34.72 144.917 34.72ZM168.188 40V12H180.588C182.321 12 183.894 12.4 185.308 13.2C186.721 13.9733 187.828 15.0267 188.628 16.36C189.454 17.6933 189.868 19.2133 189.868 20.92C189.868 22.6 189.454 24.1333 188.628 25.52C187.828 26.88 186.721 27.96 185.308 28.76C183.894 29.56 182.321 29.96 180.588 29.96H173.988V40H168.188ZM184.068 40L176.948 27.36L183.108 26.36L191.028 40.04L184.068 40ZM173.988 25.08H180.308C181.028 25.08 181.654 24.92 182.188 24.6C182.748 24.2533 183.174 23.7867 183.468 23.2C183.761 22.5867 183.908 21.9067 183.908 21.16C183.908 20.36 183.734 19.6667 183.388 19.08C183.041 18.4933 182.534 18.04 181.868 17.72C181.201 17.3733 180.428 17.2 179.548 17.2H173.988V25.08ZM199.795 40V12H218.715V17.28H205.635V34.72H218.995V40H199.795ZM202.595 28.28V23.16H216.795V28.28H202.595Z"
                fill="black"
              />
            </svg>
          </NavLink>
        </li>
        <li className={styles.link_container}>
          <div className={styles.nav_link}>
            <NavLink to="products">Товары</NavLink>
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
            <NavLink to="services">Услуги</NavLink>
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
            <NavLink to="help">Помощь</NavLink>
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
            <NavLink to="about">О нас</NavLink>
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
          sidebar_open ? "" : styles.sidebar_visible
        }`}
      >
        <div className={styles.sidebar_content}>
          <div className={styles.sidebar_nav_links}>
            <li
              className={
                link_toggled.products
                  ? `${styles.link_container} ${styles.link_toggled}`
                  : styles.link_container
              }
            >
              <div
                className={styles.nav_link}
                onClick={() => toggle_link("products")}
              >
                <div>Товары</div>
                <GoChevronDown className={styles.chevron} />
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
                  ? `${styles.link_container} ${styles.link_toggled}`
                  : styles.link_container
              }
            >
              <div
                className={styles.nav_link}
                onClick={() => toggle_link("services")}
              >
                <div>Услуги</div>
                <GoChevronDown className={styles.chevron} />
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
                  ? `${styles.link_container} ${styles.link_toggled}`
                  : styles.link_container
              }
            >
              <div
                className={styles.nav_link}
                onClick={() => toggle_link("help")}
              >
                <div>Помощь</div>
                <GoChevronDown className={styles.chevron} />
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
                  ? `${styles.link_container} ${styles.link_toggled}`
                  : styles.link_container
              }
            >
              <div
                className={styles.nav_link}
                onClick={() => toggle_link("about")}
              >
                <div>О нас</div>
                <GoChevronDown className={styles.chevron} />
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
          <div className={styles.logging}>
            <div className={styles.log}>войти</div>
            <div className={styles.log}>создать аккаунт</div>
          </div>
        </div>
        <div className={styles.sidebar_footer}>
          <div className={styles.copyright}>
            ©2024 Mustore, all rights reserved
          </div>
          <div className={styles.social_media_container}>
            <FaYoutube className={styles.social_media} />
            <FaVk className={styles.social_media} />
            <FaTelegram className={styles.social_media} />
            <BsInstagram className={styles.social_media} />
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Menu;
