import styles from "./UploadProductTempl.module.scss";
import { Link, Outlet } from "react-router-dom";
import { FC, useEffect, useState } from "react";
import { GoChevronLeft, GoChevronRight } from "react-icons/go";

interface UploadProductTemplProps {
  kind: "song" | "beat" | "cover" | "text";
}

const UploadProductTempl: FC<UploadProductTemplProps> = ({ kind }) => {
  const [translated_kind, set_translated_kind] = useState("");
  const [step_class_names, set_step_class_names] = useState({
    step_one: `${styles.current_step}`,
    step_two: ``,
    step_three: ``,
  });
  const [nav_button_class_names, set_nav_button_class_names] = useState({
    prev: `${styles.prev_button}`,
    next: `${styles.next_disabled}`,
  });
  const [next_button_name, set_next_button_name] = useState("продолжить");
  const [prev_link, set_prev_link] = useState("products");

  useEffect(() => {
    switch (kind) {
      case "song":
        set_translated_kind("Новая песня");
        break;
      case "beat":
        set_translated_kind("Новый бит");
        break;
      case "cover":
        set_translated_kind("Новая обложка");
        break;
      case "text":
        set_translated_kind("Новый текст");
        break;
    }
  }, [kind]);

  return (
    <div className={styles.upload_product}>
      <div className={styles.content}>
        <h2 className={styles.h2}>{translated_kind}</h2>
        <div className={styles.steps}>
          <div className={`${styles.step} ${step_class_names.step_one}`}>
            <div className={styles.step_number}>1</div>
            <div className={styles.step_name}>Общая информация</div>
          </div>
          <div className={`${styles.step} ${step_class_names.step_two}`}>
            <div className={styles.step_number}>2</div>
            <div className={styles.step_name}>Критерии</div>
          </div>
          <div className={`${styles.step} ${step_class_names.step_three}`}>
            <div className={styles.step_number}>3</div>
            <div className={styles.step_name}>Обложка и аудиофайлы</div>
          </div>
        </div>
        <Outlet />
        <div className={styles.nav_buttons}>
          <Link
            to={`../../${prev_link}`}
            className={`${styles.nav_button} ${nav_button_class_names.prev}`}
          >
            <GoChevronLeft
              className={`${styles.chevron} ${styles.chevron_left}`}
            />
            <p>вернуться</p>
          </Link>
          <div
            className={`${styles.nav_button} ${nav_button_class_names.next}`}
          >
            <p>{next_button_name}</p>
            <GoChevronRight
              className={`${styles.chevron} ${styles.chevron_right}`}
            />
          </div>
        </div>
        <hr className={styles.divider} />
        <p className={styles.footnote}>
          <span>*</span> - Обязательное поле
        </p>
      </div>
    </div>
  );
};

export default UploadProductTempl;
