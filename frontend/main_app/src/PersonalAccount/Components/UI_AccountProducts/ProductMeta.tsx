import { MusicKey } from "../../../types/types";
import styles from "./ProductMeta.module.scss";
import { FC, useEffect, useRef, useState } from "react";
import { FiEdit3 } from "react-icons/fi";
import { GoChevronDown } from "react-icons/go";

interface ProudctMetaProps {
  likes_count: string;
  listenings_count: string;
  lyric: string;
  moods: string[];
  music_key: MusicKey;
  name: string;
  price: string;
  primary_genre: string;
  secondary_genre?: string;
  sex: string;
  song_id: string;
  tempo: string;
}

const arr_to_string = (arr: string[]) => {
  let result: string = "";
  arr.forEach((string, idx) => {
    result += string;
    if (idx < arr.length - 1) {
      result += ", ";
    }
  });
  return result;
};

const format_price = (number_string: string) => {
  return number_string.replace(/\B(?=(\d{3})+(?!\d))/g, " ");
};

const format_music_key = (music_key: string) => {
  let formatted_music_key =
    music_key.charAt(0).toUpperCase() +
    music_key
      .slice(1)
      .replace("_sharp", "♯")
      .replace("_flat", "♭")
      .replace(/_/g, " ");

  return formatted_music_key;
};

const format_sex = (sex: string) => {
  let result = "";
  switch (sex) {
    case "Male":
      result = "Мужской";
      break;
    case "Female":
      result = "Женский";
      break;
  }

  return result;
};

const format_lyric = (lyric: string) => {
  const regex = /^[A-Za-zА-Яа-я]+$/u;
  let substrings: string[] = [];
  let current_substring = "";

  for (let i = 0; i < lyric.length; i++) {
    if (regex.test(lyric.charAt(i))) {
      if (lyric.charAt(i) === lyric.charAt(i).toUpperCase()) {
        if (!current_substring) {
          current_substring = lyric.charAt(i);
        } else {
          substrings.push(current_substring);
          current_substring = lyric.charAt(i);
        }
      } else {
        current_substring += lyric.charAt(i);
      }
    } else {
      current_substring += lyric.charAt(i);
    }
  }

  substrings.push(current_substring);
  return substrings;
};

const format_title = (title: string) => {
  return title.split(" ");
};

type ExpandCollapse = "развернуть" | "свернуть";

const ProductMeta: FC<ProudctMetaProps> = ({
  likes_count,
  listenings_count,
  lyric,
  moods,
  music_key,
  name,
  price,
  primary_genre,
  secondary_genre,
  sex,
  tempo,
  song_id,
}) => {
  const lyric_ref = useRef<HTMLDivElement>(null);
  const [visible_lyric_height, set_visible_lyric_height] = useState("20.5rem");
  const [expand_collapse, set_expand_collapse] =
    useState<ExpandCollapse>("развернуть");

  const handle_expand_collapse = () => {
    if (expand_collapse === "развернуть") {
      set_expand_collapse("свернуть");
      set_visible_lyric_height(
        `${lyric_ref.current?.clientHeight.toString()}px`
      );
    } else {
      set_expand_collapse("развернуть");
      set_visible_lyric_height("20.5rem");
    }
  };

  useEffect(() => {
    if (expand_collapse === "свернуть") {
      set_visible_lyric_height(
        `${lyric_ref.current?.clientHeight.toString()}px`
      );
    }
  }, [song_id, expand_collapse]);

  return (
    <div className={styles.product_meta}>
      <div className={styles.meta_header}>
        <p className={styles.name}>
          {format_title(name).map((word, idx) => {
            return idx === 0 ? (
              <span key={idx}>
                <span>{word}&nbsp;</span>
                <br />
              </span>
            ) : (
              <span key={idx}>{word}&nbsp;</span>
            );
          })}
        </p>
        <FiEdit3 className={styles.edit_icon} />
        <p className={styles.edit}>редактировать</p>
      </div>
      <ul className={styles.meta_list}>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Жанр: </p>
          <p className={styles.meta_value}>
            {primary_genre}
            {secondary_genre ? `, ${secondary_genre}` : ""}
          </p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Mood: </p>
          <p className={styles.meta_value}>{arr_to_string(moods)}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Лайки: </p>
          <p className={styles.meta_value}>{likes_count}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Прослушивания: </p>
          <p className={styles.meta_value}>{listenings_count}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Цена: </p>
          <p className={styles.meta_value}>{format_price(price)} ₽</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Темп: </p>
          <p className={styles.meta_value}>{tempo} BPM</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Тональность: </p>
          <p
            className={styles.meta_value}
            style={{ height: "1.375rem" }}
          >
            {format_music_key(music_key)}
          </p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Вокал: </p>
          <p className={styles.meta_value}>{format_sex(sex)}</p>
        </li>
        <li
          className={`${styles.meta_item} ${styles.meta_lyric}`}
          style={{ height: visible_lyric_height }}
        >
          <p className={styles.meta_type}>Текст: </p>
          <div
            ref={lyric_ref}
            className={styles.meta_value}
          >
            {format_lyric(lyric).map((string, idx) => {
              return (
                <p key={idx}>
                  {string}
                  <br />
                </p>
              );
            })}
          </div>
          <p
            className={styles.expand_collapse}
            onClick={handle_expand_collapse}
          >
            {expand_collapse}
          </p>
          <GoChevronDown
            className={styles.chevron}
            style={{
              transform: `rotate(${
                expand_collapse === "развернуть" ? "0" : "-180deg"
              })`,
            }}
            onClick={handle_expand_collapse}
          />
        </li>
      </ul>
    </div>
  );
};

export default ProductMeta;
