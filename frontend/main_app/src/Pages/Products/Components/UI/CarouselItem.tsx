import styles from "./CarouselItem.module.scss";
import { FC, useMemo } from "react";
import { CarouselItem } from "../../../../types/types";
import { FaRegHeart } from "react-icons/fa";
import { PiShoppingCartFill } from "react-icons/pi";

interface ClassNames {
  item_container: string;
  image_wrapper: string;
  text_info: string;
  name_and_author: string;
  name: string;
  author: string;
  price_and_actions: string;
  price: string;
  like_icon: string;
  likes_amount: string;
  cart_icon: string;
}

interface CarouselItemProps {
  carousel_type: "recommended" | "new";
  carousel_items: CarouselItem;
}

const CarouselItem: FC<CarouselItemProps> = ({
  carousel_type,
  carousel_items,
}) => {
  const formatted_price = carousel_items.price.toLocaleString("ru-RU");
  const class_names = useMemo<ClassNames>(() => {
    const base_class_names: ClassNames = {
      item_container: `${styles.item_container}`,
      image_wrapper: `${styles.image_wrapper}`,
      text_info: `${styles.text_info}`,
      name_and_author: `${styles.name_and_author}`,
      name: `${styles.name}`,
      author: `${styles.author}`,
      price_and_actions: `${styles.price_and_actions}`,
      price: `${styles.price}`,
      like_icon: `${styles.like_icon}`,
      likes_amount: `${styles.likes_amount}`,
      cart_icon: `${styles.cart_icon}`,
    };

    switch (carousel_type) {
      case "recommended":
        break;
      case "new":
        base_class_names.item_container += ` ${styles.item_container_new}`;
        base_class_names.image_wrapper += ` ${styles.image_wrapper_new}`;
        base_class_names.text_info += ` ${styles.text_info_new}`;
        base_class_names.name_and_author += ` ${styles.name_and_author_new}`;
        base_class_names.name += ` ${styles.name_new}`;
        base_class_names.author += ` ${styles.author_new}`;
        base_class_names.price_and_actions += ` ${styles.price_and_actions_new}`;
        base_class_names.price += ` ${styles.price_new}`;
        base_class_names.like_icon += ` ${styles.like_icon_new}`;
        base_class_names.likes_amount += ` ${styles.likes_amount_new}`;
        base_class_names.cart_icon += ` ${styles.cart_icon_new}`;
    }

    return base_class_names;
  }, [carousel_type]);

  return (
    <div className={class_names.item_container}>
      <div className={class_names.image_wrapper}>
        <img
          src={carousel_items.cover_url}
          alt="album cover"
          draggable={false}
        />
      </div>
      <div className={class_names.text_info}>
        <div className={class_names.name_and_author}>
          <p className={class_names.name}>{carousel_items.name}</p>
          <p className={class_names.author}>{carousel_items.author}</p>
        </div>
        <div className={class_names.price_and_actions}>
          <p className={class_names.price}>{formatted_price} ₽</p>
          <div className={styles.action_icons}>
            <div className={styles.likes}>
              <FaRegHeart className={class_names.like_icon} />
              <p className={class_names.likes_amount}>{carousel_items.likes}</p>
            </div>
            <PiShoppingCartFill className={class_names.cart_icon} />
          </div>
        </div>
      </div>
    </div>
  );
};

export default CarouselItem;
