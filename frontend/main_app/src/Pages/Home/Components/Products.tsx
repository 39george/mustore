import styles from "./Products.module.scss";
import { FC } from "react";
import cover_4 from "../../../assets/HomePage/album_covers/ablum_cover_4.png";
import cover_5 from "../../../assets/HomePage/album_covers/ablum_cover_5.png";
import cover_6 from "../../../assets/HomePage/album_covers/ablum_cover_6.png";
import cover_7 from "../../../assets/HomePage/album_covers/ablum_cover_7.png";
import cover_8 from "../../../assets/HomePage/album_covers/ablum_cover_8.png";
import cover_9 from "../../../assets/HomePage/album_covers/ablum_cover_9.png";
import cover_10 from "../../../assets/HomePage/album_covers/ablum_cover_10.png";
import cover_11 from "../../../assets/HomePage/album_covers/ablum_cover_11.png";
import { NavLink } from "react-router-dom";

const Products: FC = () => {
  return (
    <section className={styles.products_section}>
      <h2 className={styles.h2}>Начните с готовых решений</h2>
      <div className={styles.container}>
        <div className={styles.product}>
          <div className={styles.covers_container}>
            <NavLink
              to="products"
              className={styles.cover_hover}
            >
              <div className={styles.text_content}>
                к контенту
                <div className={styles.hover_dots}>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                </div>
              </div>
            </NavLink>
            <div className={styles.image_wrapper}>
              <img
                src={cover_8}
                alt="cover_8"
                className={styles.cover_upper}
              />
            </div>
            <img
              src={cover_7}
              alt="cover_7"
              className={`${styles.covers} ${styles.cover_7}`}
            />
            <img
              src={cover_6}
              alt="cover_6"
              className={`${styles.covers} ${styles.cover_6}`}
            />
            <img
              src={cover_5}
              alt="cover_5"
              className={`${styles.covers} ${styles.cover_5}`}
            />
            <img
              src={cover_4}
              alt="cover_4"
              className={`${styles.covers} ${styles.cover_4}`}
            />
          </div>
          <hr className={styles.divider} />
          <p className={styles.product_name}>
            <span>29</span> песен
          </p>
        </div>
        <div className={styles.product}>
          <div className={styles.covers_container}>
            <NavLink
              to="products"
              className={styles.cover_hover}
            >
              <div className={styles.text_content}>
                к контенту
                <div className={styles.hover_dots}>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                </div>
              </div>
            </NavLink>
            <div className={styles.image_wrapper}>
              <img
                src={cover_9}
                alt="cover_9"
                className={styles.cover_upper}
              />
            </div>
            <img
              src={cover_7}
              alt="cover_7"
              className={`${styles.covers} ${styles.cover_7}`}
            />
            <img
              src={cover_6}
              alt="cover_6"
              className={`${styles.covers} ${styles.cover_6}`}
            />
            <img
              src={cover_5}
              alt="cover_5"
              className={`${styles.covers} ${styles.cover_5}`}
            />
            <img
              src={cover_4}
              alt="cover_4"
              className={`${styles.covers} ${styles.cover_4}`}
            />
          </div>
          <hr className={styles.divider} />
          <p className={styles.product_name}>
            <span>43</span> бита
          </p>
        </div>
        <div className={styles.product}>
          <div className={styles.covers_container}>
            <NavLink
              to="products"
              className={styles.cover_hover}
            >
              <div className={styles.text_content}>
                к контенту
                <div className={styles.hover_dots}>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                </div>
              </div>
            </NavLink>
            <div className={styles.image_wrapper}>
              <img
                src={cover_10}
                alt="cover_10"
                className={styles.cover_upper}
              />
            </div>
            <img
              src={cover_7}
              alt="cover_7"
              className={`${styles.covers} ${styles.cover_7}`}
            />
            <img
              src={cover_6}
              alt="cover_6"
              className={`${styles.covers} ${styles.cover_6}`}
            />
            <img
              src={cover_5}
              alt="cover_5"
              className={`${styles.covers} ${styles.cover_5}`}
            />
            <img
              src={cover_4}
              alt="cover_4"
              className={`${styles.covers} ${styles.cover_4}`}
            />
          </div>
          <hr className={styles.divider} />
          <p className={styles.product_name}>
            <span>37</span> обложек
          </p>
        </div>
        <div className={styles.product}>
          <div className={styles.covers_container}>
            <NavLink
              to="products"
              className={styles.cover_hover}
            >
              <div className={styles.text_content}>
                к контенту
                <div className={styles.hover_dots}>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                  <div className={styles.hover_dot}></div>
                </div>
              </div>
            </NavLink>
            <div className={styles.image_wrapper}>
              <img
                src={cover_11}
                alt="cover_11"
                className={styles.cover_upper}
              />
            </div>
            <img
              src={cover_7}
              alt="cover_7"
              className={`${styles.covers} ${styles.cover_7}`}
            />
            <img
              src={cover_6}
              alt="cover_6"
              className={`${styles.covers} ${styles.cover_6}`}
            />
            <img
              src={cover_5}
              alt="cover_5"
              className={`${styles.covers} ${styles.cover_5}`}
            />
            <img
              src={cover_4}
              alt="cover_4"
              className={`${styles.covers} ${styles.cover_4}`}
            />
          </div>
          <hr className={styles.divider} />
          <p className={styles.product_name}>
            <span>19</span> текстов
          </p>
        </div>
      </div>
    </section>
  );
};

export default Products;
