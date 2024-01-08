import { IoSearch } from "react-icons/io5";
import styles from "./MainContentProducts.module.scss";
import { FC } from "react";
import { GoChevronDown } from "react-icons/go";
import { music_keys } from "../helpers";
import useGenresMoodsApi from "../../../hooks/useGenresMoodsApi";

const MainContentProducts: FC = () => {
  const {
    data: genres,
    error: genres_error,
    retry: retry_genres,
  } = useGenresMoodsApi("genres");
  const {
    data: moods,
    error: moods_error,
    retry: retry_moods,
  } = useGenresMoodsApi("tags");

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
        <ul className={`${styles.block} ${styles.genres_block}`}>
          <li className={styles.block_title}>Жанр</li>
          {genres_error ? (
            <li>{genres_error}</li>
          ) : (
            <ul className={styles.genres_content}>
              {genres.map((genre, index) => {
                return (
                  <li key={index}>
                    <input
                      type="checkbox"
                      id={genre}
                      name="genre"
                      className={styles.checkbox}
                    />
                    <label htmlFor={genre}>{genre}</label>
                  </li>
                );
              })}
            </ul>
          )}
        </ul>
        <div className={`${styles.block} ${styles.bpm_block}`}>
          <p className={styles.block_title}>Темп (BPM)</p>
          <div className={styles.range_bar}>
            <div className={styles.range_bar_low}>
              <div className={styles.knob}></div>
              <p className={styles.low_number}>40</p>
            </div>
            <div className={styles.range_bar_line}></div>
            <div className={styles.range_bar_high}>
              <div className={styles.knob}></div>
              <p className={styles.high_number}>320</p>
            </div>
          </div>
        </div>
        <ul className={`${styles.block} ${styles.music_keys_block}`}>
          <li className={styles.block_title}>Тональность</li>
          {music_keys.map((key, index) => {
            return (
              <li key={index}>
                <input
                  type="checkbox"
                  id={key}
                  name="key"
                  className={styles.checkbox}
                />
                <label htmlFor={key}>{key}</label>
              </li>
            );
          })}
        </ul>
        <ul className={`${styles.block} ${styles.moods_block}`}>
          <li className={styles.block_title}>Mood</li>
          {moods_error ? (
            <li>{moods_error}</li>
          ) : (
            <ul className={styles.moods_content}>
              {moods.map((mood, index) => {
                return (
                  <li key={index}>
                    <input
                      type="checkbox"
                      id={mood}
                      name="mood"
                      className={styles.checkbox}
                    />
                    <label htmlFor={mood}>{mood}</label>
                  </li>
                );
              })}
            </ul>
          )}
        </ul>
      </div>
    </div>
  );
};

export default MainContentProducts;
