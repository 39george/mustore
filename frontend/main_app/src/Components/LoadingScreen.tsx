import styles from "./LoadingScreen.module.scss";
import { FC } from "react";

const LoadingScreen: FC = () => {
  return (
    <div className={styles.loading_screen}>
      <div className={styles.content}>
        <div className={styles.loader_small}></div>
        <p className={styles.p_loading}>Один момент!</p>
        <h1 className={styles.title}>
          HARMONY<span>.</span>
          SPHERE
        </h1>
        <p
          className={styles.p_loading}
          style={{ opacity: 0.6, letterSpacing: "0.1em" }}
        >
          наш сервис загружается...
        </p>
      </div>
    </div>
  );
};

export default LoadingScreen;
