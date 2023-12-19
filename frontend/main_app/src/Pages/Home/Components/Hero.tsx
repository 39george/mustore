import styles from "./Hero.module.scss";

const Hero = () => {
  return (
    <div className={styles.hero_section}>
      <div className={styles.text_block}>
        <div className={styles.title}>MUSTORE</div>
        <div className={styles.tagline}>
          Сервис для тех, кто хочет найти или предложить свои{" "}
          <span>музыкальные решения.</span>
        </div>
      </div>
    </div>
  );
};

export default Hero;
