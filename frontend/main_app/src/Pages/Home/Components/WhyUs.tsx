import styles from "./WhyUs.module.scss";
import { FC } from "react";
import check_mark from "../../../assets/svg/check_mark.svg";

const WhyUs: FC = () => {
  return (
    <section className={styles.why_us_section}>
      <div className={styles.main_content}>
        <h2 className={styles.h2}>Почему стоит выбрать MUSTORE</h2>
        <div className={styles.reason}>
          <div className={styles.reason_title}>
            <img
              src={check_mark}
              alt="check mark icon"
            />
            <h3 className={styles.h3}>Сервис удобный для всех</h3>
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
            <img
              src={check_mark}
              alt="check mark icon"
            />
            <h3 className={styles.h3}>Причина 2</h3>
          </div>
          <p className={styles.reason_description}>
            Здесь должно быть описание причины, по которой людям понравится наш
            сервис
          </p>
        </div>
        <div className={styles.reason}>
          <div className={styles.reason_title}>
            <img
              src={check_mark}
              alt="check mark icon"
            />
            <h3 className={styles.h3}>Причина 3</h3>
          </div>
          <p className={styles.reason_description}>
            Здесь должно быть описание причины, по которой людям понравится наш
            сервис
          </p>
        </div>
        <div className={styles.reason}>
          <div className={styles.reason_title}>
            <img
              src={check_mark}
              alt="check mark icon"
            />
            <h3 className={styles.h3}>Причина 4</h3>
          </div>
          <p className={styles.reason_description}>
            Здесь должно быть описание причины, по которой людям понравится наш
            сервис
          </p>
        </div>
      </div>
    </section>
  );
};

export default WhyUs;
