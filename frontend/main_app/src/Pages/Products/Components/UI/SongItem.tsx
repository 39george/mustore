import styles from "./SongItem.module.scss";
import { FC, memo } from "react";
import { FaRegHeart } from "react-icons/fa";
import { BsPlayCircle } from "react-icons/bs";
import AudioWavesIcon from "../../../../UI/AudioWavesIcon";
import { HiDotsHorizontal } from "react-icons/hi";
import { ISongItem } from "../../../../types/types";

const SongItem: FC<ISongItem> = memo(
  ({ author, name, price, cover_url, primary_genre, moods }) => {
    const formatted_price = price.toLocaleString("ru-RU");

    return (
      <div className={styles.song_item}>
        <div className={styles.image_wrapper}>
          <img
            src={cover_url}
            alt="album cover"
            draggable={false}
          />
          <div className={styles.image_hover}>
            <div className={styles.decor_and_info}>
              <AudioWavesIcon
                width="25"
                height="25"
                fill="#FEFEFE"
              />
              <HiDotsHorizontal className={styles.more_info_icon} />
            </div>
            <BsPlayCircle className={styles.play_icon} />
          </div>
        </div>
        <p className={styles.name}>{name}</p>
        <p className={styles.author}>by {author}</p>
        <hr className={styles.divider} />
        <div className={styles.price_container}>
          <p className={styles.price_name}>цена</p>
          <p className={styles.price}>{formatted_price} ₽</p>
        </div>
        <div className={styles.tags}>
          <p className={styles.tag}>{primary_genre}</p>
          <p className={styles.tag}>{moods[0]}</p>
          <p className={styles.tag}>{moods[1]}</p>
        </div>
        <div className={styles.action_buttons}>
          <div className={styles.buy_button}>купить</div>
          <div className={styles.like_container}>
            <FaRegHeart className={styles.like_icon} />
          </div>
        </div>
      </div>
    );
  }
);

export default SongItem;
