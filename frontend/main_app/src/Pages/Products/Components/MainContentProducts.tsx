import { IoSearch } from "react-icons/io5";
import styles from "./MainContentProducts.module.scss";
import { FC, useEffect } from "react";
import { GoChevronDown } from "react-icons/go";
import axios from "axios";

const API_URL = import.meta.env.VITE_REACT_APP_API_URL;

const MainContentProducts: FC = () => {
  useEffect(() => {
    const get_genre_list = async () => {
      try {
        const response = await fetch(`${API_URL}/open/genres`);
        const data = await response.json();
        console.log(data);
      } catch (error) {
        console.log("error message:", error);
      }
    };

    get_genre_list();
  }, []);

  return (
    <div className={styles.main_seciton}>
      <div className={styles.left_bar}>
        <div className={`${styles.block}${styles.search_block}`}>
          <input
            type="text"
            className={styles.input}
            placeholder="Поиск..."
          />
          <IoSearch className={styles.search_icon} />
        </div>
        <div className={`${styles.block} ${styles.filter_block}`}>
          <p>Популярные</p>
          <GoChevronDown className={styles.filter_chevron} />
        </div>
        <ul className={`${styles.block} ${styles.sex_block}`}>
          <li className={styles.block_title}>Вокал</li>
          <li>
            <input
              type="radio"
              id="any"
              name="sex"
              defaultChecked
              className={styles.checkbox}
            />
            <label htmlFor="any">Любой</label>
          </li>
          <li>
            <input
              type="radio"
              id="male"
              name="sex"
              className={styles.checkbox}
            />
            <label htmlFor="male">Мужской</label>
          </li>
          <li>
            <input
              type="radio"
              id="female"
              name="sex"
              className={styles.checkbox}
            />
            <label htmlFor="female">Женский</label>
          </li>
        </ul>
        <ul className={`${styles.block} ${styles.genre_block}`}>
          <li>
            <input
              type="checkbox"
              id="pop"
              name="genre"
            />
            <label htmlFor="pop">Поп</label>
          </li>
        </ul>
      </div>
    </div>
  );
};

export default MainContentProducts;
