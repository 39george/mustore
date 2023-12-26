import styles from "./Services.module.scss";
import { FC } from "react";
import { FaLongArrowAltRight } from "react-icons/fa";
import { NavLink } from "react-router-dom";
import songwriting from "../../../assets/svg/songwriting.svg";
import mixing from "../../../assets/svg/mixing.svg";
import beatmaking from "../../../assets/svg/beatmaking.svg";
import ghostwriting from "../../../assets/svg/ghostwriting.svg";
import cover_design from "../../../assets/svg/cover_design.svg";

const Services: FC = () => {
  return (
    <section className={styles.services_section}>
      <div className={styles.main_content}>
        <h2 className={styles.h2}>Или закажите необходимую услугу</h2>
        <div className={styles.services_container}>
          <div className={styles.service}>
            <img
              src={songwriting}
              alt="service songwriting"
              className={styles.service_icon}
            />
            <div className={styles.service_text}>
              <h3 className={styles.h3}>Создание песни</h3>
              <p className={styles.description}>
                Здесь должно быть краткое описание сервиса, его суть и смысл в
                одном - двух предложениях
              </p>
              <div className={styles.link_container}>
                <NavLink
                  to="services"
                  className={styles.link}
                >
                  найти профессионала
                </NavLink>
                <FaLongArrowAltRight className={styles.arrow_icon} />
              </div>
            </div>
          </div>
          <div className={styles.service}>
            <img
              src={mixing}
              alt="service songwriting"
              className={styles.service_icon}
            />
            <div className={styles.service_text}>
              <h3 className={styles.h3}>Сведение / Мастеринг</h3>
              <p className={styles.description}>
                Здесь должно быть краткое описание сервиса, его суть и смысл в
                одном - двух предложениях
              </p>
              <div className={styles.link_container}>
                <NavLink
                  to="services"
                  className={styles.link}
                >
                  найти профессионала
                </NavLink>
                <FaLongArrowAltRight className={styles.arrow_icon} />
              </div>
            </div>
          </div>
          <div className={styles.service}>
            <img
              src={beatmaking}
              alt="service beatmaking"
              className={styles.service_icon}
            />
            <div className={styles.service_text}>
              <h3 className={styles.h3}>Написание бита</h3>
              <p className={styles.description}>
                Здесь должно быть краткое описание сервиса, его суть и смысл в
                одном - двух предложениях
              </p>
              <div className={styles.link_container}>
                <NavLink
                  to="services"
                  className={styles.link}
                >
                  найти профессионала
                </NavLink>
                <FaLongArrowAltRight className={styles.arrow_icon} />
              </div>
            </div>
          </div>
          <div className={styles.service}>
            <img
              src={ghostwriting}
              alt="service ghostriting"
              className={styles.service_icon}
            />
            <div className={styles.service_text}>
              <h3 className={styles.h3}>Гострайтинг</h3>
              <p className={styles.description}>
                Здесь должно быть краткое описание сервиса, его суть и смысл в
                одном - двух предложениях
              </p>
              <div className={styles.link_container}>
                <NavLink
                  to="services"
                  className={styles.link}
                >
                  найти профессионала
                </NavLink>
                <FaLongArrowAltRight className={styles.arrow_icon} />
              </div>
            </div>
          </div>
          <div className={styles.service}>
            <img
              src={cover_design}
              alt="service cover designk"
              className={styles.service_icon}
            />
            <div className={styles.service_text}>
              <h3 className={styles.h3}>Дизайн обложки</h3>
              <p className={styles.description}>
                Здесь должно быть краткое описание сервиса, его суть и смысл в
                одном - двух предложениях
              </p>
              <div className={styles.link_container}>
                <NavLink
                  to="services"
                  className={styles.link}
                >
                  найти профессионала
                </NavLink>
                <FaLongArrowAltRight className={styles.arrow_icon} />
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Services;
