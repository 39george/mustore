import styles from "./Services.module.scss";
import { FC } from "react";
import { FaLongArrowAltRight } from "react-icons/fa";
import { NavLink } from "react-router-dom";
import Mixing from "../../../assets/svg/Mixing";
import Songwriting from "../../../assets/svg/Songwriting";
import BeatMaking from "../../../assets/svg/BeatMaking";
import Ghostwriting from "../../../assets/svg/Ghostwriting";
import CoverDesign from "../../../assets/svg/CoverDesign";

const Services: FC = () => {
  return (
    <section className={styles.services_section}>
      <div className={styles.main_content}>
        <h2 className={styles.h2}>Или закажите необходимую услугу</h2>
        <div className={styles.services_container}>
          <div className={styles.service}>
            <Songwriting />
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
            <Mixing />
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
            <BeatMaking />
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
            <Ghostwriting />
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
            <CoverDesign />
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
