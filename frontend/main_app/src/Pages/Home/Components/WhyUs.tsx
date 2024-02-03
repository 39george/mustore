import styles from "./WhyUs.module.scss";
import { FC } from "react";
import underline from "../../../assets/svg/underline.svg";
import { PiHandshakeFill } from "react-icons/pi";
import { FaUserShield } from "react-icons/fa";
import { BsLightningChargeFill } from "react-icons/bs";
import { FaMusic } from "react-icons/fa6";

const WhyUs: FC = () => {
  return (
    <section className={styles.why_us_section}>
      <div className={styles.main_content}>
        <h2 className={styles.h2}>
          Почему стоит выбрать{" "}
          <span>
            HARMONY.SPHERE
            <img
              src={underline}
              alt="underline"
              className={styles.underline}
            />
          </span>
        </h2>
        <div className={styles.reasons_container}>
          <div className={styles.reason}>
            <div className={styles.reason_title}>
              <h3 className={styles.h3}>Сервис удобный для всех</h3>
              <PiHandshakeFill className={styles.reason_icon} />
            </div>
            <p className={styles.reason_description}>
              Мы создали платформу, на которой комфортно всем: и желающим найти
              для себя лучшее творческое решение - от одельных элементов до
              полностью готовой песни - и тем, кто мечтал начать зарабатывать на
              своих творческих идеях прямо сейчас
            </p>
          </div>
          <div className={styles.reason}>
            <div className={styles.reason_title}>
              <h3 className={styles.h3}>Надежность</h3>
              <FaUserShield className={styles.reason_icon} />
            </div>
            <p className={styles.reason_description}>
              Наш сервис – это крепость для вашего творчества. Нет более важного
              аспекта в цифровом пространстве, чем надежность. Мы гарантируем,
              что все ваши сделки, а также хранение и передача музыкального
              материала защищены надежными технологиями.
            </p>
          </div>
          <div className={styles.reason}>
            <div className={styles.reason_title}>
              <h3 className={styles.h3}>Потрясающая скорость</h3>
              <BsLightningChargeFill className={styles.reason_icon} />
            </div>
            <p className={styles.reason_description}>
              В этом мире скорость означает все. Мы используем последние
              технологические достижения, чтобы обеспечить вас сверхбыстрым
              доступом к покупкам и загрузкам. Позвольте вашему творчеству
              двигаться с потрясающей скоростью.
            </p>
          </div>
          <div className={styles.reason}>
            <div className={styles.reason_title}>
              <h3 className={styles.h3}>Большие возможности</h3>
              <FaMusic className={styles.reason_icon} />
            </div>
            <p className={styles.reason_description}>
              Мы предлагаем безграничное пространство для продажи, покупки и
              коллаборации. Воспользуйтесь широким спектром инструментов и
              услуг, которые помогут вам реализовать самые амбициозные
              музыкальные проекты.
            </p>
          </div>
        </div>
      </div>
    </section>
  );
};

export default WhyUs;
