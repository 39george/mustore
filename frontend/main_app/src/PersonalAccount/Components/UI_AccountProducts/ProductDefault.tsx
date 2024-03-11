import styles from "./ProductDefault.module.scss";
import { FiPlus } from "react-icons/fi";
import { FC } from "react";

const ProductDefault: FC = () => {
  return (
    <div className={styles.product_default}>
      <h2 className={styles.h2}>Здесь пока ничего нет!</h2>
      <div className={styles.upload_product}>
        <p className={styles.upload_product_p}>загрузить новый товар</p>
        <FiPlus className={styles.plus_icon} />
      </div>
    </div>
  );
};

export default ProductDefault;
