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
            MUSTORE
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
              для себя лучшее творческое решение от одельных элементов до
              полностью готовой песни, и тем, кто мечтал начать зарабатывать на
              своих творческих идеях
            </p>
          </div>
          <div className={styles.reason}>
            <div className={styles.reason_title}>
              <h3 className={styles.h3}>Надежность</h3>
              <FaUserShield className={styles.reason_icon} />
            </div>
            <p className={styles.reason_description}>
              Мы создали платформу, на которой комфортно всем: и желающим найти
              для себя лучшее творческое решение от одельных элементов до
              полностью готовой песни, и тем, кто мечтал начать зарабатывать на
              своих творческих идеях
            </p>
          </div>
          <div className={styles.reason}>
            <div className={styles.reason_title}>
              <h3 className={styles.h3}>Потрясающая скорость</h3>
              <BsLightningChargeFill className={styles.reason_icon} />
            </div>
            <p className={styles.reason_description}>
              Мы создали платформу, на которой комфортно всем: и желающим найти
              для себя лучшее творческое решение от одельных элементов до
              полностью готовой песни, и тем, кто мечтал начать зарабатывать на
              своих творческих идеях
            </p>
          </div>
          <div className={styles.reason}>
            <div className={styles.reason_title}>
              <h3 className={styles.h3}>Большие возможности</h3>
              <FaMusic className={styles.reason_icon} />
            </div>
            <p className={styles.reason_description}>
              Мы создали платформу, на которой комфортно всем: и желающим найти
              для себя лучшее творческое решение от одельных элементов до
              полностью готовой песни, и тем, кто мечтал начать зарабатывать на
              своих творческих идеях
            </p>
          </div>
        </div>
      </div>
    </section>
  );
};

export default WhyUs;
