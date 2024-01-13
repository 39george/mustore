import { IoSearch } from "react-icons/io5";
import styles from "./MainContentProducts.module.scss";
import { FC, useEffect, useRef, useState } from "react";
import { GoChevronDown } from "react-icons/go";
import { music_keys } from "../helpers";
import useGenresMoodsApi from "../../../hooks/useGenresMoodsApi";
import useCheckboxState from "../../../hooks/useCheckboxState";
import { CheckedItems } from "../../../types/types";
import DraggableSlider from "./UI/DraggableSlider";
import { song_items } from "./UI/content_dummies";
import SongItem from "./UI/SongItem";

interface StickyState {
  position: "absolute" | "fixed" | "relative" | "static" | "sticky";
  top: string;
  bottom: string;
}

interface ScrollConsts {
  height_diff_viewport_main_content: number;
  height_diff_viewport_left_bar: number;
  main_content: HTMLDivElement | null;
  left_bar: HTMLDivElement | null;
}

type LeftBarStates =
  | "absolute_top"
  | "absolute_bottom"
  | "sticky_top"
  | "sticky_bottom"
  | "absolute_offset";

type ScrollDirection = "down" | "up";

const MainContentProducts: FC = () => {
  const main_section_ref = useRef<HTMLDivElement>(null);
  const wrapper_ref = useRef<HTMLDivElement>(null);
  const left_bar_ref = useRef<HTMLDivElement>(null);
  const [sticky, set_sticky] = useState<StickyState>({
    position: "absolute",
    top: "",
    bottom: "",
  });
  const last_scroll_top = useRef(0);
  const last_scroll_direction = useRef<ScrollDirection>("down");
  const last_offset = useRef(0);
  const [nav_bar_height, set_nav_bar_height] = useState(70);
  const [scroll_consts, set_scroll_consts] = useState<ScrollConsts>({
    height_diff_viewport_main_content: 0,
    height_diff_viewport_left_bar: 0,
    main_content: main_section_ref.current,
    left_bar: left_bar_ref.current,
  });
  const [left_bar_state, set_left_bar_state] =
    useState<LeftBarStates>("absolute_top");
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

  // Handle scroll and change left_bar position
  useEffect(() => {
    if (main_section_ref.current && left_bar_ref.current) {
      set_scroll_consts({
        height_diff_viewport_main_content:
          window.innerHeight - main_section_ref.current.offsetHeight,
        height_diff_viewport_left_bar:
          window.innerHeight - left_bar_ref.current.offsetHeight,
        main_content: main_section_ref.current,
        left_bar: left_bar_ref.current,
      });
    }
  }, [window.innerHeight, main_section_ref.current, left_bar_ref.current]);

  const check_sticky = () => {
    // Check for null
    if (!scroll_consts.main_content || !scroll_consts.left_bar) {
      return;
    }

    // Variables declaration
    const dist_from_top_viewport_to_main_content =
      scroll_consts.main_content.getBoundingClientRect().top;

    const dist_from_top_viewport_to_left_bar =
      scroll_consts.left_bar.getBoundingClientRect().top;

    const current_scroll = window.scrollY;

    const scroll_direction =
      current_scroll > last_scroll_top.current ? "down" : "up";

    const main_content_fully_scrolled_to_bottom = () => {
      return (
        dist_from_top_viewport_to_main_content <=
        scroll_consts.height_diff_viewport_main_content
      );
    };

    const main_content_fully_scrolled_to_top = () => {
      return dist_from_top_viewport_to_main_content >= nav_bar_height;
    };

    const left_bar_fully_scrolled_to_bottom = () => {
      return (
        dist_from_top_viewport_to_left_bar <=
        scroll_consts.height_diff_viewport_left_bar
      );
    };

    const left_bar_fully_scrolled_to_top = () => {
      return dist_from_top_viewport_to_left_bar >= nav_bar_height + 32;
    };

    // Check scroll direction changing
    if (scroll_direction !== last_scroll_direction.current) {
      last_offset.current = Math.round(
        dist_from_top_viewport_to_main_content -
          dist_from_top_viewport_to_left_bar
      );
      last_scroll_direction.current = scroll_direction;
    }

    last_scroll_top.current = current_scroll;

    // Main logic
    switch (left_bar_state) {
      case "absolute_top":
        set_sticky({
          position: "absolute",
          top: "",
          bottom: "",
        });
        break;
      case "absolute_bottom":
        set_sticky({
          position: "absolute",
          top: "",
          bottom: "0px",
        });
        break;
      case "sticky_top":
        set_sticky({
          position: "fixed",
          top: `${nav_bar_height}px`,
          bottom: "",
        });
        break;
      case "sticky_bottom":
        set_sticky({
          position: "fixed",
          top: "",
          bottom: "0px",
        });
        break;
      case "absolute_offset":
        set_sticky({
          position: "absolute",
          top: `${-last_offset.current - 32}px`,
          bottom: "",
        });
        break;
    }

    if (main_content_fully_scrolled_to_bottom()) {
      set_left_bar_state("absolute_bottom");
    } else {
      if (left_bar_fully_scrolled_to_bottom()) {
        set_left_bar_state("sticky_bottom");
      }
      if (scroll_direction === "down") {
        if (left_bar_state === "sticky_top") {
          if (!left_bar_fully_scrolled_to_bottom()) {
            set_left_bar_state("absolute_offset");
          }
        }
      }
      if (scroll_direction === "up") {
        if (left_bar_fully_scrolled_to_top()) {
          set_left_bar_state("sticky_top");
        }
        if (main_content_fully_scrolled_to_top()) {
          set_left_bar_state("absolute_top");
        }
        if (left_bar_state === "sticky_bottom") {
          if (!left_bar_fully_scrolled_to_top()) {
            set_left_bar_state("absolute_offset");
          }
        }
      }
    }
  };

  useEffect(() => {
    window.addEventListener("scroll", check_sticky);

    return () => {
      window.removeEventListener("scroll", check_sticky);
    };
  }, [scroll_consts, left_bar_state, nav_bar_height]);

  useEffect(() => {
    if (window.innerWidth > 1010) {
      set_nav_bar_height(83);
    }
  }, [window.innerWidth]);

  return (
    <div
      ref={main_section_ref}
      className={styles.main_seciton}
    >
      <div
        ref={wrapper_ref}
        className={styles.left_bar_wrapper}
      >
        <div
          ref={left_bar_ref}
          className={styles.left_bar}
          style={{
            position: `${sticky.position}`,
            top: `${sticky.top}`,
            bottom: `${sticky.bottom}`,
          }}
        >
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
                          onChange={() =>
                            handle_mood_checkbox_change(`${mood}`)
                          }
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
