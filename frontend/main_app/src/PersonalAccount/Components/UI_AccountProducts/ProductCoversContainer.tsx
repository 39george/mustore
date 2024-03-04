import { GoChevronLeft, GoChevronRight } from "react-icons/go";
import styles from "./ProductCoversContainer.module.scss";
import { FC } from "react";
import { IProduct } from "../../../types/types";

interface ProductCoversContainerProps {
  products: IProduct[];
}

const ProductCoversContainer: FC<ProductCoversContainerProps> = ({
  products,
}) => {
  return (
    <div className={styles.covers_carousel}>
      <div className={`${styles.nav_button} ${styles.button_left}`}>
        <GoChevronLeft className={styles.nav_chevron} />
      </div>
      <div className={styles.product_covers_container}>
        {products.map((product, idx) => {
          const translate_amount = 3 * idx;
          const cover_opacity = idx < 3 ? 1 : 0;
          const overlay_opacity = 0.3 * idx;
          return (
            <div
              className={`${styles.image_wrapper}`}
              style={{
                width: `calc(16rem - 2rem * ${idx})`,
                height: `calc(16rem - 2rem * ${idx})`,
                transform: `translateX(${translate_amount}rem)`,
                opacity: `${cover_opacity}`,
                zIndex: `${products.length - idx}`,
              }}
              key={idx}
            >
              <div
                className={styles.overlay}
                style={{ opacity: `${overlay_opacity}` }}
              ></div>
              <img
                src={product.key}
                alt={`product${idx + 1}`}
                draggable={false}
              />
            </div>
          );
        })}
      </div>
      <div className={`${styles.nav_button} ${styles.button_right}`}>
        <GoChevronRight className={styles.nav_chevron} />
      </div>
    </div>
  );
};

export default ProductCoversContainer;
