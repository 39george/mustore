import styles from "./CarouselItem.module.scss";
import { FC } from "react";
import { CarouselItemProps } from "../../../../types/types";
import { GoHeartFill } from "react-icons/go";
import { GoHeart } from "react-icons/go";
import { PiShoppingCartFill } from "react-icons/pi";

const CarouselItem: FC<CarouselItemProps> = (item) => {
  const formatted_price = item.price.toLocaleString("ru-RU");

  return (
    <div className={styles.item_container}>
      <div className={styles.image_wrapper}>
        <img
          src={item.cover_url}
          alt="album cover"
        />
      </div>
      <div className={styles.text_info}>
        <p className={styles.name}>{item.name}</p>
        <p className={styles.author}>{item.author}</p>
        <p className={styles.price}>{formatted_price} ₽</p>
        <div className={styles.action_icons}>
          <div className={styles.likes}>
            <GoHeart className={styles.like_icon} />
            <p className={styles.likes_amount}>{item.likes}</p>
          </div>
          <PiShoppingCartFill className={styles.cart_icon} />
        </div>
      </div>
    </div>
  );
};

export default CarouselItem;
