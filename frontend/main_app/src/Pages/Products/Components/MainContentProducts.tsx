import { IoSearch } from "react-icons/io5";
import styles from "./MainContentProducts.module.scss";
import { FC, useEffect, useState } from "react";
import { GoChevronDown } from "react-icons/go";
import axios, { AxiosError } from "axios";
import { API_URL } from "../../../config";

const MAX_RETRIES = 3;
const RETRY_DELAY_MS = 400;

type GenreOrMood = string[];

const MainContentProducts: FC = () => {
  const [genres, set_genres] = useState<GenreOrMood>([]);
  const [genres_error, set_genres_error] = useState<string | null>(null);

  // GET genres request
  useEffect(() => {
    const wait = (ms: number) =>
      new Promise((resolve) => setTimeout(resolve, ms));

    const get_genre_list = async (attempts: number = 1) => {
      try {
        const response = await axios.get<GenreOrMood>(`${API_URL}/open/genres`);
        if (Array.isArray(response.data)) {
          set_genres(response.data);
        } else {
          console.error("Unexpected response type:", response.data);
        }
      } catch (error) {
        if (axios.isAxiosError(error)) {
          if (error.response) {
            console.error(
              "API Error:",
              error.response.status,
              error.response.data
            );

            if (attempts < MAX_RETRIES) {
              await wait(RETRY_DELAY_MS);
              get_genre_list(attempts + 1);
            } else {
              handle_axios_error(error);
            }
          } else if (error.request) {
            if (attempts < MAX_RETRIES) {
              await wait(RETRY_DELAY_MS);
              get_genre_list(attempts + 1);
            } else {
              set_genres_error(
                "No response from server. Please, check your internet connection and try again"
              );
            }
          } else {
            set_genres_error("Error in setting up the request.");
            console.error("API Error: Reqest setup error:", error.message);
          }
        } else {
          set_genres_error("An unexpected error occured.");
          console.error("Non-Axios:", error);
        }
      }
    };

    const handle_axios_error = (error: AxiosError) => {
      if (error.response) {
        switch (error.response.status) {
          case 400:
            set_genres_error(
              "Bad input, please check your request and try again"
            );
            break;
          case 500:
            set_genres_error("Internal server error. Please try again later.");
            break;
          default:
            set_genres_error(
              "An unexpected error occured. Please, try again later."
            );
            break;
        }
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
