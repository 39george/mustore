import { IoSearch } from "react-icons/io5";
import styles from "./MainContentProducts.module.scss";
import { FC, useState } from "react";
import { GoChevronDown } from "react-icons/go";
import { music_keys } from "../helpers";
import useGenresMoodsApi from "../../../hooks/useGenresMoodsApi";
import useCheckboxState from "../../../hooks/useCheckboxState";
import { CheckedItems } from "../../../types/types";
import DraggableSlider from "./UI/DraggableSlider";
import { song_items } from "./UI/content_dummies";
import SongItem from "./UI/SongItem";

const MainContentProducts: FC = () => {
  const { data: genres, error: genres_error } = useGenresMoodsApi("genres");
  const { data: moods, error: moods_error } = useGenresMoodsApi("tags");
  const {
    checked_items: checked_genres,
    handle_checkbox_change: handle_genres_checkbox_change,
  } = useCheckboxState();
  const {
    checked_items: checked_music_key,
    handle_checkbox_change: handle_music_key_checkbox_change,
  } = useCheckboxState();
  const {
    checked_items: checked_mood,
    handle_checkbox_change: handle_mood_checkbox_change,
  } = useCheckboxState();
  const [checked_sex, set_checked_sex] = useState<CheckedItems>({ any: true });

  const handle_sex_checkbox_change = (sex: string) => {
    set_checked_sex({ [sex]: true });
  };

  return (
    <div className={styles.main_seciton}>
      <div className={styles.left_bar}>
        <div className={`${styles.block} ${styles.search_block}`}>
          <input
            type="text"
            name="search"
            className={styles.input}
            placeholder="Поиск..."
          />
          <IoSearch className={styles.search_icon} />
        </div>
        <div className={`${styles.block} ${styles.filter_block}`}>
          <p className={styles.block_title}>Популярные</p>
          <GoChevronDown className={styles.filter_chevron} />
        </div>
        <ul className={`${styles.block} ${styles.sex_block}`}>
          <li className={styles.block_title}>Вокал</li>
          <ul className={styles.sex_content}>
            <li className={styles.li_item}>
              <label
                htmlFor="any"
                className={styles.custom_checkbox}
              >
                <input
                  type="checkbox"
                  id="any"
                  name="sex"
                  className={styles.checkbox}
                  onChange={() => handle_sex_checkbox_change("any")}
                  checked={checked_sex["any"] || false}
                />
                <span className={styles.checkmark}></span>
              </label>
              <label
                htmlFor="any"
                className={`${styles.label} ${
                  checked_sex["any"] ? styles.checked_label : ""
                }`}
              >
                Любой
              </label>
            </li>
            <li className={styles.li_item}>
              <label
                htmlFor="male"
                className={styles.custom_checkbox}
              >
                <input
                  type="checkbox"
                  id="male"
                  name="sex"
                  className={styles.checkbox}
                  onChange={() => handle_sex_checkbox_change("male")}
                  checked={checked_sex["male"] || false}
                />
                <span className={styles.checkmark}></span>
              </label>
              <label
                htmlFor="male"
                className={`${styles.label} ${
                  checked_sex["male"] ? styles.checked_label : ""
                }`}
              >
                Мужской
              </label>
            </li>
            <li className={styles.li_item}>
              <label
                htmlFor="female"
                className={styles.custom_checkbox}
              >
                <input
                  type="checkbox"
                  id="female"
                  name="sex"
                  className={styles.checkbox}
                  onChange={() => handle_sex_checkbox_change("female")}
                  checked={checked_sex["female"] || false}
                />
                <span className={styles.checkmark}></span>
              </label>
              <label
                htmlFor="female"
                className={`${styles.label} ${
                  checked_sex["female"] ? styles.checked_label : ""
                }`}
              >
                Женский
              </label>
            </li>
          </ul>
        </ul>
        <ul className={`${styles.block} ${styles.genres_block}`}>
          <li className={styles.block_title}>Жанр</li>
          {genres_error ? (
            <li className={styles.error}>{genres_error}</li>
          ) : (
            <ul className={styles.genres_content}>
              {genres.map((genre, index) => {
                return (
                  <li
                    key={index}
                    className={styles.li_item}
                  >
                    <label
                      htmlFor={genre}
                      className={styles.custom_checkbox}
                    >
                      <input
                        type="checkbox"
                        id={genre}
                        name="genre"
                        className={styles.checkbox}
                        onChange={() =>
                          handle_genres_checkbox_change(`${genre}`)
                        }
                        checked={checked_genres[genre] || false}
                      />
                      <span className={styles.checkmark}></span>
                    </label>
                    <label
                      htmlFor={genre}
                      className={`${styles.label} ${
                        checked_genres[genre] ? styles.checked_label : ""
                      }`}
                    >
                      {genre}
                    </label>
                  </li>
                );
              })}
            </ul>
          )}
        </ul>
        <div className={`${styles.block} ${styles.bpm_block}`}>
          <p className={styles.block_title}>Темп (BPM)</p>
          <DraggableSlider />
        </div>
        <ul className={`${styles.block} ${styles.music_keys_block}`}>
          <li className={styles.block_title}>Тональность</li>
          <ul className={styles.music_keys_content}>
            {music_keys.map((key, index) => {
              return (
                <li
                  key={index}
                  className={styles.li_item}
                >
                  <label
                    htmlFor={key}
                    className={styles.custom_checkbox}
                  >
                    <input
                      type="checkbox"
                      id={key}
                      name="key"
                      className={styles.checkbox}
                      onChange={() =>
                        handle_music_key_checkbox_change(`${key}`)
                      }
                      checked={checked_music_key[key] || false}
                    />
                    <span className={styles.checkmark}></span>
                  </label>
                  <label
                    htmlFor={key}
                    className={`${styles.label} ${
                      checked_music_key[key] ? styles.checked_label : ""
                    }`}
                  >
                    {key}
                  </label>
                </li>
              );
            })}
          </ul>
        </ul>
        <ul className={`${styles.block} ${styles.moods_block}`}>
          <li className={styles.block_title}>Mood песни</li>
          {moods_error ? (
            <li className={styles.error}>{moods_error}</li>
          ) : (
            <ul className={styles.moods_content}>
              {moods.map((mood, index) => {
                return (
                  <li
                    key={index}
                    className={styles.li_item}
                  >
                    <label
                      htmlFor={mood}
                      className={styles.custom_checkbox}
                    >
                      <input
                        type="checkbox"
                        id={mood}
                        name="mood"
                        className={styles.checkbox}
                        onChange={() => handle_mood_checkbox_change(`${mood}`)}
                        checked={checked_mood[mood] || false}
                      />
                      <span className={styles.checkmark}></span>
                    </label>
                    <label
                      htmlFor={mood}
                      className={`${styles.label} ${
                        checked_mood[mood] ? styles.checked_label : ""
                      }`}
                    >
                      {mood}
                    </label>
                  </li>
                );
              })}
            </ul>
          )}
        </ul>
      </div>
      <div className={styles.products_container}>
        {song_items.map((item) => {
          return (
            <SongItem
              key={item.id}
              name={item.name}
              cover_url={item.cover_url}
              author={item.author}
              likes={item.likes}
              listenings={item.listenings}
              price={item.price}
            />
          );
        })}
      </div>
    </div>
  );
};

export default MainContentProducts;
