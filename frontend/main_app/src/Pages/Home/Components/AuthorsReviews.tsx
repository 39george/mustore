import styles from "./AuthorsReviews.module.scss";
import { FC } from "react";
import author_1 from "../../../assets/HomePage/author_1.png";
import author_2 from "../../../assets/HomePage/author_2.png";
import author_3 from "../../../assets/HomePage/author_3.png";

const AuthorsReviews: FC = () => {
  return (
    <section className={styles.authors_reviews_section}>
      <h2 className={styles.h2}>Что о нас говорят авторы</h2>
      <div className={styles.reviews_container}>
        <div className={styles.review}>
          <div className={styles.image_wrapper}>
            <img
              src={author_1}
              alt="author avatar"
            />
          </div>
          <div className={styles.text_content}>
            <p className={styles.description}>
              Благодаря HARMONY.SHPERE моё искусство нашло своих ценителей
            </p>
            <p className={styles.author_name}>ALENA NAI</p>
          </div>
        </div>
        <div className={styles.review}>
          <div className={styles.image_wrapper}>
            <img
              src={author_2}
              alt="author avatar"
            />
          </div>
          <div className={styles.text_content}>
            <p className={styles.description}>
              Интуитивно понятный интерфейс и быстрые выплаты сделали мой опыт
              невероятно удобным
            </p>
            <p className={styles.author_name}>MISHA LETNIY</p>
          </div>
        </div>
        <div className={styles.review}>
          <div className={styles.image_wrapper}>
            <img
              src={author_3}
              alt="author avatar"
            />
          </div>
          <div className={styles.text_content}>
            <p className={styles.description}>
              Удобный процесс загрузки и продажи моих музыкальных услуг помогает
              мне сосредоточиться на создании качественного контента, не
              отвлекаясь на технические аспекты.
            </p>
            <p className={styles.author_name}>OLEGUS PRO</p>
          </div>
        </div>
      </div>
    </section>
  );
};

export default AuthorsReviews;
