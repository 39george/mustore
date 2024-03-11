import styles from "./ProductCoversContainer.module.scss";
import { GoChevronLeft, GoChevronRight } from "react-icons/go";
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

const define_width_height = (
  idx: number,
  product_idx: number,
  cover_size: number
) => {
  let result: string = "";
  idx < product_idx
    ? (result = `calc(${cover_size}rem + 2rem * (${idx} - ${product_idx}))`)
    : (result = `calc(${cover_size}rem - 2rem * ${idx - product_idx})`);

  return result;
};

const ProductCoversContainer: FC<ProductCoversContainerProps> = ({
  products,
  product_idx,
  change_active_product,
}) => {
  const [after_init_render, set_after_init_render] = useState(``);
  const [z_index, set_z_index] = useState(
    products.map((_, idx) => products.length - idx)
  );
  const [left_shift, set_left_shift] = useState({
    idx_less_than_product_idx: window.innerWidth < 685 ? 1 : 2,
    idx_greater_than_product_idx: window.innerWidth < 685 ? 3 : 4,
  });
  const [cover_size, set_cover_size] = useState(
    window.innerWidth < 685 ? 12 : 16
  );
  const [small_sreen, set_small_screen] = useState(window.innerWidth < 531);

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

  useEffect(() => {
    const handle_resize = () => {
      set_small_screen(window.innerWidth < 531);

      if (!small_sreen) {
        set_left_shift({
          idx_less_than_product_idx: window.innerWidth < 685 ? 1 : 2,
          idx_greater_than_product_idx: window.innerWidth < 685 ? 3 : 4,
        });
        set_cover_size(window.innerWidth < 685 ? 12 : 16);
      }
    };

    window.addEventListener("resize", handle_resize);

    return () => {
      window.removeEventListener("resize", handle_resize);
    };
  }, [small_sreen]);

  useEffect(() => {
    if (small_sreen) {
      set_cover_size(8);
    } else {
      set_cover_size(window.innerWidth < 685 ? 12 : 16);
    }
  }, [small_sreen, window.innerWidth]);

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
            const calculated_left_shift =
              idx < product_idx
                ? left_shift.idx_less_than_product_idx * (idx - product_idx)
                : left_shift.idx_greater_than_product_idx * (idx - product_idx);
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
                  width: `${define_width_height(
                    idx,
                    product_idx,
                    cover_size
                  )} `,
                  height: `${define_width_height(
                    idx,
                    product_idx,
                    cover_size
                  )} `,
                  left: `${calculated_left_shift}rem`,
                  opacity: `${cover_opacity}`,
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
