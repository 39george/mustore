import { IoSearch } from "react-icons/io5";
import styles from "./MainContentProducts.module.scss";
import { FC, useEffect, useRef, useState } from "react";
import { GoChevronDown } from "react-icons/go";
import { music_keys } from "../music_keys";
import useGenresMoodsApi from "../../../hooks/API/useGenresMoodsApi";
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

type FilteredType = "filtered_genres" | "filtered_moods";

interface FilteredResults {
  filtered_genres: string[];
  filtered_moods: string[];
}

interface SearchTerms {
  genres: string;
  moods: string;
}

interface ExpandedBlocks {
  sex: boolean;
  genres: boolean;
  tempo: boolean;
  music_key: boolean;
  moods: boolean;
}

type ExpandingBlocks = "sex" | "genres" | "tempo" | "music_key" | "moods";

const MainContentProducts: FC = () => {
  // HTML refs
  const main_section_ref = useRef<HTMLDivElement>(null);
  const wrapper_ref = useRef<HTMLDivElement>(null);
  const left_bar_ref = useRef<HTMLDivElement>(null);
  // Consts for left bar sticky logic
  const [sticky, set_sticky] = useState<StickyState>({
    position: "absolute",
    top: "",
    bottom: "",
  });
  const last_scroll_top = useRef(0);
  const last_scroll_direction = useRef<ScrollDirection>("down");
  const last_offset = useRef(0);
  const [nav_bar_height, set_nav_bar_height] = useState(83);
  const scroll_consts = useRef<ScrollConsts>({
    height_diff_viewport_main_content: 0,
    height_diff_viewport_left_bar: 0,
    main_content: main_section_ref.current,
    left_bar: left_bar_ref.current,
  });
  const left_bar_state = useRef<LeftBarStates>("absolute_top");
  const left_bar_prev_state = useRef<LeftBarStates>("absolute_top");
  const previous_dist_from_top_viewport_to_left_bar = useRef(0);
  // Consts for different screens styles
  const [is_small_screen, set_is_small_screen] = useState(
    window.innerWidth <= 768
  );
  const [is_filters_hidden, set_is_filters_hidden] = useState(true);
  const [expanded_blocks, set_expanded_blocks] = useState<ExpandedBlocks>({
    sex: false,
    genres: false,
    tempo: false,
    music_key: false,
    moods: false,
  });
  // Data consts
  const { data: genres, error: genres_error } = useGenresMoodsApi("genres");
  const { data: moods, error: moods_error } = useGenresMoodsApi("moods");
  const [filtered_results, set_filtered_results] = useState<FilteredResults>({
    filtered_genres: [],
    filtered_moods: [],
  });
  const [search_terms, set_search_terms] = useState<SearchTerms>({
    genres: "",
    moods: "",
  });
  const {
    checked_items: checked_genres,
    set_checked_items: set_checked_genres,
    handle_checkbox_change: handle_genres_checkbox_change,
  } = useCheckboxState();
  const {
    checked_items: checked_music_key,
    set_checked_items: set_checked_music_key,
    handle_checkbox_change: handle_music_key_checkbox_change,
  } = useCheckboxState();
  const {
    checked_items: checked_moods,
    set_checked_items: set_checked_moods,
    handle_checkbox_change: handle_moods_checkbox_change,
  } = useCheckboxState();
  const [checked_sex, set_checked_sex] = useState<CheckedItems>({ any: true });
  const music_symbols = ["♭", "♯"];
  const [is_iphone] = useState(/iPhone/.test(navigator.userAgent));

  // Changing checkbox for sex block
  const handle_sex_checkbox_change = (sex: string) => {
    set_checked_sex({ [sex]: true });
  };

  // Expand blocks
  const handle_blocks_expand = (name: ExpandingBlocks) => {
    if (!is_small_screen) {
      return;
    }

    set_expanded_blocks((prev) => ({
      ...prev,
      [name]: !prev[name],
    }));
  };

  // Check if a checked object has any `true` value
  const no_true_values = (obj: CheckedItems) => {
    return Object.values(obj).every((value) => value === false);
  };

  // Set all values of a chekced object to `false`
  const set_all_to_false = (
    e: React.MouseEvent<HTMLLIElement>,
    obj: CheckedItems,
    obj_kind: "genres" | "music_key" | "moods"
  ) => {
    e.stopPropagation();

    let new_obj: CheckedItems = {};
    Object.keys(obj).forEach((key) => {
      new_obj[key] = false;
    });

    switch (obj_kind) {
      case "genres":
        set_checked_genres(new_obj);
        break;
      case "music_key":
        set_checked_music_key(new_obj);
        break;
      case "moods":
        set_checked_moods(new_obj);
        break;
    }
  };

  // Set initial filtered genres
  useEffect(() => {
    set_filtered_results({
      filtered_genres: genres,
      filtered_moods: moods,
    });
  }, [genres, moods]);

  // Filter genres or moods
  useEffect(() => {
    if (search_terms.genres || search_terms.genres === "") {
      handle_genres_moods_search(
        genres,
        search_terms.genres,
        "filtered_genres"
      );
    }
    if (search_terms.moods || search_terms.moods === "") {
      handle_genres_moods_search(moods, search_terms.moods, "filtered_moods");
    }
  }, [search_terms]);

  const handle_genres_moods_search = (
    kind: string[],
    search_term: string,
    output: FilteredType
  ) => {
    let results = kind.filter((item) =>
      item.toLowerCase().includes(search_term.toLowerCase())
    );
    if (search_term) {
      set_filtered_results((prev) => ({
        ...prev,
        [output]: results,
      }));
    } else {
      set_filtered_results((prev) => ({
        ...prev,
        [output]: kind,
      }));
    }
  };

  // Formatting strings in keys array
  const format_key_with_symbols = (keys: string) => {
    return keys.split("").map((char, idx) => {
      if (music_symbols.includes(char)) {
        return (
          <span
            key={idx}
            className={styles.music_symbol}
          >
            {char}
          </span>
        );
      }
      return char;
    });
  };

  // Setting nav bar position logic
  const set_left_bar_position = () => {
    // Check for null
    if (
      !scroll_consts.current.main_content ||
      !scroll_consts.current.left_bar
    ) {
      return;
    }

    // Variables declaration
    const dist_from_top_viewport_to_main_content = Math.round(
      scroll_consts.current.main_content.getBoundingClientRect().top
    );

    const dist_from_top_viewport_to_left_bar = Math.round(
      scroll_consts.current.left_bar.getBoundingClientRect().top
    );

    const current_scroll = window.scrollY;

    const scroll_direction =
      current_scroll > last_scroll_top.current ? "down" : "up";

    const main_content_fully_scrolled_to_bottom = () => {
      return (
        dist_from_top_viewport_to_main_content <=
        scroll_consts.current.height_diff_viewport_main_content
      );
    };

    const main_content_fully_scrolled_to_top = () => {
      return dist_from_top_viewport_to_main_content >= nav_bar_height;
    };

    // console.log("dist", dist_from_top_viewport_to_left_bar);
    // console.log("prev", previous_dist_from_top_viewport_to_left_bar.current);
    const left_bar_fully_scrolled_to_bottom = () => {
      let difference =
        dist_from_top_viewport_to_left_bar -
        previous_dist_from_top_viewport_to_left_bar.current;
      if (
        dist_from_top_viewport_to_left_bar + difference <=
        scroll_consts.current.height_diff_viewport_left_bar
      ) {
        return true;
      }
      return (
        dist_from_top_viewport_to_left_bar <=
        scroll_consts.current.height_diff_viewport_left_bar
      );
    };

    const left_bar_fully_scrolled_to_top = () => {
      return dist_from_top_viewport_to_left_bar >= nav_bar_height + 32;
    };

    // Check scroll direction changing and set offset for left bar height
    if (scroll_direction !== last_scroll_direction.current) {
      last_offset.current =
        dist_from_top_viewport_to_main_content -
        dist_from_top_viewport_to_left_bar;

      last_scroll_direction.current = scroll_direction;
    }

    last_scroll_top.current = current_scroll;

    // Main logic
    if (scroll_direction === "down") {
      if (main_content_fully_scrolled_to_bottom()) {
        left_bar_state.current = "absolute_bottom";
      }
      if (
        left_bar_fully_scrolled_to_bottom() &&
        !main_content_fully_scrolled_to_bottom()
      ) {
        left_bar_state.current = "sticky_bottom";
      }
      if (left_bar_state.current === "sticky_top") {
        if (!left_bar_fully_scrolled_to_bottom()) {
          left_bar_state.current = "absolute_offset";
        }
      }
    }

    if (scroll_direction === "up") {
      if (main_content_fully_scrolled_to_top()) {
        left_bar_state.current = "absolute_top";
      }
      if (
        left_bar_fully_scrolled_to_top() &&
        !main_content_fully_scrolled_to_top()
      ) {
        left_bar_state.current = "sticky_top";
      }
      if (left_bar_state.current === "sticky_bottom") {
        if (!left_bar_fully_scrolled_to_top()) {
          left_bar_state.current = "absolute_offset";
        }
      }
    }

    if (left_bar_prev_state.current !== left_bar_state.current) {
      switch (left_bar_state.current) {
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
            bottom: "0",
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
            bottom: "0",
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

      left_bar_prev_state.current = left_bar_state.current;
    }
    previous_dist_from_top_viewport_to_left_bar.current =
      dist_from_top_viewport_to_left_bar;
  };

  // Handling all resize events
  useEffect(() => {
    const maybe_set_left_bar_position = () => {
      if (window.innerWidth > 768) {
        set_left_bar_position();
      }
    };

    const handle_resize = () => {
      // Set new distances for left bar and main content
      if (main_section_ref.current && left_bar_ref.current) {
        scroll_consts.current = {
          height_diff_viewport_main_content:
            window.innerHeight - main_section_ref.current.offsetHeight,
          height_diff_viewport_left_bar:
            window.innerHeight - left_bar_ref.current.offsetHeight,
          main_content: main_section_ref.current,
          left_bar: left_bar_ref.current,
        };
      }

      // Set new nav bar height
      if (window.innerWidth <= 1010) {
        set_nav_bar_height(70);
      } else {
        set_nav_bar_height(83);
      }

      // Check if small screen
      set_is_small_screen(window.innerWidth <= 768);

      // Add / remove event listener "scroll"
      if (window.innerWidth > 768) {
        window.addEventListener("scroll", maybe_set_left_bar_position);
      } else {
        window.removeEventListener("scroll", maybe_set_left_bar_position);
      }
    };

    handle_resize();

    window.addEventListener("resize", handle_resize);
    return () => {
      window.removeEventListener("resize", handle_resize);
      window.removeEventListener("scroll", maybe_set_left_bar_position);
    };
  }, [
    scroll_consts.current.left_bar?.offsetHeight,
    checked_genres,
    checked_music_key,
    checked_moods,
  ]);

  useEffect(() => {
    if (left_bar_ref.current) {
      const rect = left_bar_ref.current.getBoundingClientRect();
      if (
        rect.top >= 0 &&
        rect.left >= 0 &&
        rect.bottom <=
          (window.innerHeight || document.documentElement.clientHeight) &&
        rect.right <=
          (window.innerWidth || document.documentElement.clientWidth)
      ) {
      }
    }
  }, [left_bar_ref.current]);

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
          style={
            is_small_screen
              ? { position: "relative", top: "", bottom: "" }
              : {
                  position: `${sticky.position}`,
                  top: `${sticky.top}`,
                  bottom: `${sticky.bottom}`,
                }
          }
        >
          <div className={styles.search_and_sort}>
            <div className={`${styles.block} ${styles.search_block}`}>
              <input
                type="search"
                name="search"
                className={styles.global_search}
                placeholder="Поиск по названию, автору..."
              />
              <IoSearch className={styles.search_icon} />
            </div>
            <div className={`${styles.block} ${styles.sort_block}`}>
              <p className={styles.block_title}>Сначала попоулярные</p>
              <GoChevronDown className={styles.chevron} />
            </div>
          </div>
          <div className={styles.filters_container}>
            {is_small_screen && (
              <div
                className={`${styles.block} ${styles.filters_block}`}
                onClick={() => set_is_filters_hidden(!is_filters_hidden)}
              >
                <p className={styles.block_title}>Фильтры</p>
              </div>
            )}
            <div
              className={`${styles.rest_blocks} ${
                is_small_screen && is_filters_hidden
                  ? styles.rest_blocks_hidden
                  : ""
              }`}
            >
              <ul
                className={`${styles.block} ${styles.sex_block} ${
                  expanded_blocks.sex ? `${styles.sex_expanded}` : ""
                }`}
                onClick={() => handle_blocks_expand("sex")}
              >
                <li className={styles.block_title}>
                  <p>Вокал</p>
                  {is_small_screen && (
                    <GoChevronDown className={styles.chevron} />
                  )}
                </li>
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
              <ul
                className={`${styles.block} ${styles.genres_block} ${
                  expanded_blocks.genres && `${styles.genres_moods_expanded}`
                } ${
                  !no_true_values(checked_genres) &&
                  expanded_blocks.genres &&
                  `${styles.genres_moods_height_checked}`
                }`}
                onClick={() => handle_blocks_expand("genres")}
              >
                <li className={styles.block_title}>
                  <p>Жанр</p>
                  <form
                    className={styles.form_search}
                    style={{
                      display:
                        expanded_blocks.genres || !is_small_screen
                          ? "flex"
                          : "none",
                    }}
                    onSubmit={(e) => e.preventDefault()}
                  >
                    <input
                      type="search"
                      name="search"
                      placeholder="Поиск"
                      value={search_terms.genres}
                      onClick={(e) => e.stopPropagation()}
                      onChange={(e) => {
                        set_search_terms((prev) => ({
                          ...prev,
                          genres: e.target.value,
                        }));
                        set_left_bar_position();
                      }}
                    />
                  </form>
                  {is_small_screen && (
                    <GoChevronDown className={styles.chevron} />
                  )}
                </li>
                {genres_error ? (
                  <li className={styles.error}>{genres_error}</li>
                ) : (
                  <ul className={styles.genres_content}>
                    {filtered_results.filtered_genres.map((genre, index) => {
                      return (
                        <li
                          key={index}
                          className={styles.li_item}
                          onClick={set_left_bar_position}
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
                {!no_true_values(checked_genres) && (
                  <li
                    className={styles.uncheck_all}
                    onClick={(e) =>
                      set_all_to_false(e, checked_genres, "genres")
                    }
                  >
                    отменить выбор
                  </li>
                )}
              </ul>
              <div
                className={`${styles.block} ${styles.bpm_block} ${
                  expanded_blocks.tempo ? `${styles.tempo_expanded}` : ""
                }`}
                onClick={() => handle_blocks_expand("tempo")}
              >
                <div className={styles.block_title}>
                  <p>Темп (BPM)</p>
                  {is_small_screen && (
                    <GoChevronDown className={styles.chevron} />
                  )}
                </div>
                <DraggableSlider />
              </div>
              <ul
                className={`${styles.block} ${styles.music_keys_block} ${
                  expanded_blocks.music_key &&
                  `${styles.music_key_expanded} ${
                    is_iphone && styles.music_key_iphone
                  }`
                } ${
                  !no_true_values(checked_music_key) &&
                  expanded_blocks.music_key &&
                  `${styles.music_key_height_checked} ${
                    is_iphone && styles.music_key_checked_iphone
                  }`
                }`}
                onClick={() => handle_blocks_expand("music_key")}
              >
                <li className={styles.block_title}>
                  <p>Тональность</p>
                  {is_small_screen && (
                    <GoChevronDown className={styles.chevron} />
                  )}
                </li>
                <ul className={styles.music_keys_content}>
                  {music_keys.map((key, index) => {
                    return (
                      <li
                        key={index}
                        className={styles.li_item}
                        onClick={set_left_bar_position}
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
                          {format_key_with_symbols(key)}
                        </label>
                      </li>
                    );
                  })}
                </ul>
                {!no_true_values(checked_music_key) && (
                  <li
                    className={styles.uncheck_all}
                    onClick={(e) =>
                      set_all_to_false(e, checked_music_key, "music_key")
                    }
                  >
                    отменить выбор
                  </li>
                )}
              </ul>
              <ul
                className={`${styles.block} ${styles.moods_block} ${
                  expanded_blocks.moods && `${styles.genres_moods_expanded}`
                } ${
                  !no_true_values(checked_moods) &&
                  expanded_blocks.moods &&
                  `${styles.genres_moods_height_checked}`
                }`}
                onClick={() => handle_blocks_expand("moods")}
              >
                <li className={styles.block_title}>
                  <p>Mood</p>
                  <form
                    className={styles.form_search}
                    style={{
                      display:
                        expanded_blocks.moods || !is_small_screen
                          ? "flex"
                          : "none",
                    }}
                    onSubmit={(e) => e.preventDefault()}
                  >
                    <input
                      type="search"
                      name="search"
                      placeholder="Поиск"
                      value={search_terms.moods}
                      onClick={(e) => e.stopPropagation()}
                      onChange={(e) => {
                        set_search_terms((prev) => ({
                          ...prev,
                          moods: e.target.value,
                        }));
                        set_left_bar_position();
                      }}
                    />
                  </form>
                  {is_small_screen && (
                    <GoChevronDown className={styles.chevron} />
                  )}
                </li>
                {moods_error ? (
                  <li className={styles.error}>{moods_error}</li>
                ) : (
                  <ul className={styles.moods_content}>
                    {filtered_results.filtered_moods.map((mood, index) => {
                      return (
                        <li
                          key={index}
                          className={styles.li_item}
                          onClick={set_left_bar_position}
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
                                handle_moods_checkbox_change(`${mood}`)
                              }
                              checked={checked_moods[mood] || false}
                            />
                            <span className={styles.checkmark}></span>
                          </label>
                          <label
                            htmlFor={mood}
                            className={`${styles.label} ${
                              checked_moods[mood] ? styles.checked_label : ""
                            }`}
                          >
                            {mood}
                          </label>
                        </li>
                      );
                    })}
                  </ul>
                )}
                {!no_true_values(checked_moods) && (
                  <li
                    className={styles.uncheck_all}
                    onClick={(e) => set_all_to_false(e, checked_moods, "moods")}
                  >
                    отменить выбор
                  </li>
                )}
              </ul>
            </div>
          </div>
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
