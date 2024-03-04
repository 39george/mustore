import { MusicKey } from "../../../types/types";
import styles from "./ProductMeta.module.scss";
import { FC } from "react";

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
    case "male":
      result = "Мужской";
      break;
    case "female":
      result = "Женский";
      break;
  }

  return result;
};

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
}) => {
  return (
    <div className={styles.product_meta}>
      <div className={styles.meta_header}>
        <p className={styles.name}>{name}</p>
        <p className={styles.edit}>редактировать</p>
      </div>
      <ul className={styles.meta_list}>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Жанр</p>
          <p className={styles.meta_value}>
            {primary_genre}
            {secondary_genre ? `, ${secondary_genre}` : ""}
          </p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Mood</p>
          <p className={styles.meta_value}>{arr_to_string(moods)}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Лайки</p>
          <p className={styles.meta_value}>{likes_count}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Прослушивания</p>
          <p className={styles.meta_value}>{listenings_count}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Цена</p>
          <p className={styles.meta_value}>{format_price(price)}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Темп</p>
          <p className={styles.meta_value}>{tempo} BPM</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Тональность</p>
          <p className={styles.meta_value}>{format_music_key(music_key)}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Вокал</p>
          <p className={styles.meta_value}>{format_sex(sex)}</p>
        </li>
        <li className={styles.meta_item}>
          <p className={styles.meta_type}>Текст</p>
          <p className={styles.meta_value}>{lyric}</p>
        </li>
      </ul>
    </div>
  );
};

export default ProductMeta;
