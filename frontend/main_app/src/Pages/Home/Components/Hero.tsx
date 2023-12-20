import styles from "./Hero.module.scss";
import bg_decor_1 from "../../../assets/Home page/bg_decor_1.png";
import bg_decor_2 from "../../../assets/Home page/bg_decor_2.png";
import bg_decor_3 from "../../../assets/Home page/bg_decor_3.png";
import bg_decor_4 from "../../../assets/Home page/bg_decor_4.png";
import cover_1 from "../../../assets/Home page/album_covers/ablum_cover_1.png";
import cover_2 from "../../../assets/Home page/album_covers/ablum_cover_2.png";
import cover_3 from "../../../assets/Home page/album_covers/ablum_cover_3.png";
import cover_4 from "../../../assets/Home page/album_covers/ablum_cover_4.png";
import cover_5 from "../../../assets/Home page/album_covers/ablum_cover_5.png";
import cover_6 from "../../../assets/Home page/album_covers/ablum_cover_6.png";
import cover_7 from "../../../assets/Home page/album_covers/ablum_cover_7.png";
import cover_8 from "../../../assets/Home page/album_covers/ablum_cover_8.png";

const Hero = () => {
  return (
    <div className={styles.hero_section}>
      <div className={styles.content_block_left}>
        <img
          src={cover_1}
          alt="cover_1"
          className={`${styles.covers} ${styles.cover_1}`}
        />
        <img
          src={cover_2}
          alt="cover_2"
          className={`${styles.covers} ${styles.cover_2}`}
        />
        <img
          src={cover_3}
          alt="cover_3"
          className={`${styles.covers} ${styles.cover_3}`}
        />
        <img
          src={cover_4}
          alt="cover_4"
          className={`${styles.covers} ${styles.cover_4}`}
        />
        <img
          className={`${styles.bg_decor} ${styles.decor_1}`}
          src={bg_decor_1}
          alt="background decor 1"
        />
        <img
          className={`${styles.bg_decor} ${styles.decor_2}`}
          src={bg_decor_2}
          alt="background decor 2"
        />
        <img
          className={`${styles.bg_decor} ${styles.decor_3}`}
          src={bg_decor_3}
          alt="background decor 3"
        />
      </div>
      <div className={styles.text_block}>
        <div className={styles.empty_space}></div>
        <div className={styles.title}>MUSTORE</div>
        <div className={styles.tagline}>
          Сервис для тех, кто хочет найти или предложить свои{" "}
          <span>музыкальные решения.</span>
        </div>
      </div>
      <div className={styles.content_block_right}>
        <img
          src={cover_5}
          alt="cover_5"
          className={`${styles.covers} ${styles.cover_5}`}
        />
        <img
          src={cover_6}
          alt="cover_6"
          className={`${styles.covers} ${styles.cover_6}`}
        />
        <img
          src={cover_7}
          alt="cover_7"
          className={`${styles.covers} ${styles.cover_7}`}
        />
        <img
          src={cover_8}
          alt="cover_8"
          className={`${styles.covers} ${styles.cover_8}`}
        />
        <img
          className={`${styles.bg_decor} ${styles.decor_3}`}
          src={bg_decor_3}
          alt="background decor 3"
        />
        <img
          className={`${styles.bg_decor} ${styles.decor_2}`}
          src={bg_decor_2}
          alt="background decor 2"
        />
        <img
          className={`${styles.bg_decor} ${styles.decor_4}`}
          src={bg_decor_4}
          alt="background decor 4"
        />
      </div>
    </div>
  );
};

export default Hero;
