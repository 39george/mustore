import { GoChevronLeft, GoChevronRight } from "react-icons/go";
import styles from "./ProductCoversContainer.module.scss";
import { FC, useEffect, useState } from "react";
import { IProduct } from "../../../types/types";

interface ProductCoversContainerProps {
  products: IProduct[];
  product_idx: number;
  change_active_product: (product: string, idx: number) => void;
}

const define_opacity = (idx: number, product_idx: number) => {
  if (idx < product_idx) {
    return product_idx - idx > 2 ? 0 : 1;
  } else {
  }
  return idx - product_idx < 3 ? 1 : 0;
};

// const define_z_index = (
//   idx: number,
//   product_idx: number,
//   products: IProduct[]
// ) => {
//   const default_z_index = products.length - idx;
//   let current_z_index: number = 0;
//   current_z_index =
//     product_idx === 0
//       ? default_z_index
//       : idx < product_idx
//       ? default_z_index + (idx - product_idx)
//       : default_z_index + product_idx;
//   return current_z_index;
// };

const ProductCoversContainer: FC<ProductCoversContainerProps> = ({
  products,
  product_idx,
  change_active_product,
}) => {
  const [after_init_render, set_after_init_render] = useState(``);
  const [z_index, set_z_index] = useState(
    products.map((_, idx) => products.length - idx)
  );

  useEffect(() => {
    setTimeout(() => {
      set_after_init_render(`${styles.product_after_animation}`);
    }, 500);
  }, []);

  const handle_carousel_shift = (
    direction: "left" | "right",
    product: string
  ) => {
    switch (direction) {
      case "left":
        if (product_idx === 0) {
          return;
        }
        change_active_product(product, product_idx - 1);
        break;
      case "right":
        if (product_idx === products.length - 1) {
          return;
        }
        change_active_product(product, product_idx + 1);
        break;
    }
  };

  useEffect(() => {
    setTimeout(() => {
      set_z_index(
        products.map((_, idx) => {
          if (product_idx === 0) {
            return products.length - idx;
          }
          if (idx < product_idx) {
            return products.length - idx + (idx - product_idx);
          } else {
            return products.length - idx + product_idx;
          }
        })
      );
    }, 100);
  }, [product_idx]);

  return (
    <div className={styles.covers_carousel}>
      <div className={styles.content_container}>
        {product_idx !== 0 && (
          <div
            className={`${styles.nav_button} ${styles.button_left}`}
            onClick={() => {
              handle_carousel_shift("left", `product${product_idx - 1}`);
            }}
          >
            <GoChevronLeft className={styles.nav_chevron} />
          </div>
        )}
        <div className={styles.product_covers_container}>
          {products.map((product, idx) => {
            const left_shift =
              idx < product_idx
                ? 2 * (idx - product_idx)
                : 4 * (idx - product_idx);
            const cover_opacity = define_opacity(idx, product_idx);
            const overlay_opacity =
              idx < product_idx
                ? 0.3 * Math.abs(idx - product_idx)
                : 0.3 * (idx - product_idx);
            const init_render_class = `product${idx}`;
            return (
              <div
                className={`${styles.image_wrapper} ${
                  idx < 3 && styles[init_render_class]
                } ${after_init_render}`}
                style={{
                  width: `${
                    idx < product_idx
                      ? `calc(16rem + 2rem * (${idx} - ${product_idx}))`
                      : `calc(16rem - 2rem * ${idx - product_idx})`
                  } `,
                  height: `${
                    idx < product_idx
                      ? `calc(16rem + 2rem * (${idx} - ${product_idx}))`
                      : `calc(16rem - 2rem * ${idx - product_idx})`
                  } `,
                  left: `${left_shift}rem`,
                  opacity: `${cover_opacity}`,
                  // zIndex: `${define_z_index(idx, product_idx, products)}`,
                  zIndex: `${z_index[idx]}`,
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
        {product_idx < products.length - 1 && (
          <div
            className={`${styles.nav_button} ${styles.button_right}`}
            onClick={() => {
              handle_carousel_shift("right", `product${product_idx + 1}`);
            }}
          >
            <GoChevronRight className={styles.nav_chevron} />
          </div>
        )}
      </div>
    </div>
  );
};

export default ProductCoversContainer;
