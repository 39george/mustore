import styles from "./SongItem.module.scss";
import { FC, memo } from "react";
import { SongItem } from "../../../../types/types";
import { BsThreeDotsVertical } from "react-icons/bs";
import { FaRegHeart } from "react-icons/fa";
import { PiHeadphonesFill, PiShoppingCartFill } from "react-icons/pi";

const SongItem: FC<SongItem> = memo((props) => {
  const formatted_price = props.price.toLocaleString("ru-RU");

  return (
    <div className={styles.song_item}>
      <div className={styles.image_wrapper}>
        <img
          src={props.cover_url}
          alt="album cover"
          draggable={false}
        />
      </div>
      <div className={styles.title}>
        <p className={styles.name}>{props.name}</p>
        <BsThreeDotsVertical className={styles.more_info_icon} />
      </div>
      <p className={styles.author}>{props.author}</p>
      <div className={styles.likes_and_listenings}>
        <div className={styles.likes}>
          <FaRegHeart className={styles.like_icon} />
          <p className={styles.likes_amount}>{props.likes}</p>
        </div>
        <div className={styles.listenings}>
          <PiHeadphonesFill className={styles.listenings_icon} />
          <p className={styles.listenings_amount}>{props.listenings}</p>
        </div>
      </div>
      <hr className={styles.divider} />
      <div className={styles.price_and_cart}>
        <p className={styles.price}>{formatted_price} ₽</p>
        <PiShoppingCartFill className={styles.cart_icon} />
      </div>
    </div>
  );
});

export default SongItem;
