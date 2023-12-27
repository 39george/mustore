import styles from "./JoinUs.module.scss";
import { FC } from "react";

const JoinUs: FC = () => {
  return (
    <section className={styles.join_us_section}>
      <div className={styles.widget}>
        <h2 className={styles.h2}>Простая реализация вашего творчества</h2>
        <div className={styles.button}>стать одним из создателей</div>
      </div>
    </section>
  );
};

export default JoinUs;
