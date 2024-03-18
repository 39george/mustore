import styles from "./ProductDefault.module.scss";
import { FiPlus } from "react-icons/fi";
import { FC } from "react";
import { NavLink } from "react-router-dom";
import { useDispatch } from "react-redux";
import { set_product_status } from "../../../state/product_status_slice";

const ProductDefault: FC = () => {
  const dispatch = useDispatch();
  const handle_click = () => {
    dispatch(set_product_status(null));
  };
  return (
    <div className={styles.product_default}>
      <h2 className={styles.h2}>Здесь пока ничего нет!</h2>
      <NavLink
        to="upload_new_product"
        className={styles.upload_product}
        onClick={handle_click}
      >
        <p className={styles.upload_product_p}>загрузить новый товар</p>
        <FiPlus className={styles.plus_icon} />
      </NavLink>
    </div>
  );
};

export default ProductDefault;
