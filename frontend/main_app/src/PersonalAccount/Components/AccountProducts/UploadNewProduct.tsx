import styles from "./UploadNewProduct.module.scss";
import { FC } from "react";
import song_icon from "../../../assets/icons/song_account.svg";
import beat_icon from "../../../assets/icons/beat_account.svg";
import text_icon from "../../../assets/icons/text_account.svg";
import cover_icon from "../../../assets/icons/cover_account.svg";
import { Link } from "react-router-dom";

interface ProductWidget {
  name: "Песню" | "Бит" | "Текст" | "Обложку";
  upload_amount: number;
  icon: string;
  link: string;
}

const product_widgets: ProductWidget[] = [
  { name: "Песню", upload_amount: 2, icon: song_icon, link: "../upload_song" },
  { name: "Бит", upload_amount: 3, icon: beat_icon, link: "../upload_song" },
  { name: "Текст", upload_amount: 0, icon: text_icon, link: "../upload_song" },
  {
    name: "Обложку",
    upload_amount: 1,
    icon: cover_icon,
    link: "../upload_song",
  },
];

const UploadNewProduct: FC = () => {
  return (
    <div className={styles.upload_new_product}>
      <div className={styles.content}>
        <h2 className={styles.h2}>Какой товар вы хотите загрузить?</h2>
        <div className={styles.product_widgets_container}>
          {product_widgets.map((widget, idx) => {
            return (
              <Link
                to={widget.link}
                className={styles.product_widget}
                key={idx}
              >
                <div className={styles.widget_text_content}>
                  <p className={styles.widget_name}>{widget.name}</p>
                  <p className={styles.widget_upload_amount}>
                    всего
                    <br className={styles.br} />
                    загружено: <span>{widget.upload_amount}</span>
                  </p>
                </div>
                <img
                  src={widget.icon}
                  alt="widget icon"
                  draggable={false}
                  className={styles.widget_icon}
                />
              </Link>
            );
          })}
        </div>
        <Link
          to="../"
          className={styles.return_link}
        >
          вернуться назад
        </Link>
      </div>
    </div>
  );
};

export default UploadNewProduct;
