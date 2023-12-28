import styles from "./Services.module.scss";
import { FC } from "react";
import { NavLink } from "react-router-dom";
import songwriting from "../../../assets/svg/songwriting.svg";
import mixing from "../../../assets/svg/mixing.svg";
import beatmaking from "../../../assets/svg/beatmaking.svg";
import ghostwriting from "../../../assets/svg/ghostwriting.svg";
import cover_design from "../../../assets/svg/cover_design.svg";
import usePageNavigation from "../../../hooks/usePageNavigation";

const Services: FC = () => {
  const handle_page_navigation = usePageNavigation();

  return (
    <section className={styles.services_section}>
      <h2 className={styles.h2}>Или закажите необходимую услугу</h2>
      <div className={styles.services_container}>
        <div className={styles.service}>
          <div className={styles.icon_wrapper}>
            <img
              src={songwriting}
              alt="service songwriting"
              className={styles.service_icon}
            />
          </div>
          <div className={styles.service_text}>
            <h3 className={styles.h3}>Создание песни</h3>
            <p className={styles.description}>
              Здесь должно быть краткое описание сервиса, его суть и смысл в
              одном - двух предложениях
            </p>
            <NavLink
              to="services"
              className={styles.link}
              onClick={() => handle_page_navigation("services")}
            >
              найти профессионала
            </NavLink>
          </div>
        </div>
        <div className={styles.service}>
          <div className={styles.icon_wrapper}>
            <img
              src={mixing}
              alt="service mixing"
              className={styles.service_icon}
            />
          </div>
          <div className={styles.service_text}>
            <h3 className={styles.h3}>Сведение / Мастеринг</h3>
            <p className={styles.description}>
              Здесь должно быть краткое описание сервиса, его суть и смысл в
              одном - двух предложениях
            </p>
            <NavLink
              to="services"
              className={styles.link}
              onClick={() => handle_page_navigation("services")}
            >
              найти профессионала
            </NavLink>
          </div>
        </div>
        <div className={styles.service}>
          <div className={styles.icon_wrapper}>
            <img
              src={beatmaking}
              alt="service beatmaking"
              className={styles.service_icon}
            />
          </div>
          <div className={styles.service_text}>
            <h3 className={styles.h3}>Написание бита</h3>
            <p className={styles.description}>
              Здесь должно быть краткое описание сервиса, его суть и смысл в
              одном - двух предложениях
            </p>
            <NavLink
              to="services"
              className={styles.link}
              onClick={() => handle_page_navigation("services")}
            >
              найти профессионала
            </NavLink>
          </div>
        </div>
        <div className={styles.service}>
          <div className={styles.icon_wrapper}>
            <img
              src={ghostwriting}
              alt="service ghostwriting"
              className={styles.service_icon}
            />
          </div>
          <div className={styles.service_text}>
            <h3 className={styles.h3}>Гострайтинг</h3>
            <p className={styles.description}>
              Здесь должно быть краткое описание сервиса, его суть и смысл в
              одном - двух предложениях
            </p>
            <NavLink
              to="services"
              className={styles.link}
              onClick={() => handle_page_navigation("services")}
            >
              найти профессионала
            </NavLink>
          </div>
        </div>
        <div className={styles.service}>
          <div className={styles.icon_wrapper}>
            <img
              src={cover_design}
              alt="service cover design"
              className={styles.service_icon}
            />
          </div>
          <div className={styles.service_text}>
            <h3 className={styles.h3}>Дизайн обложки</h3>
            <p className={styles.description}>
              Здесь должно быть краткое описание сервиса, его суть и смысл в
              одном - двух предложениях
            </p>
            <NavLink
              to="services"
              className={styles.link}
              onClick={() => handle_page_navigation("services")}
            >
              найти профессионала
            </NavLink>
          </div>
        </div>
        <div className={`${styles.service} ${styles.mock_service}`}></div>
      </div>
    </section>
  );
};

export default Services;
