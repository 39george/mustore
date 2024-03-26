import styles from "./CarouselItem.module.scss";
import { FC, memo, useMemo } from "react";
import { CarouselItem } from "../../../../types/types";
import { FaRegHeart } from "react-icons/fa";
import { BsPlayCircle } from "react-icons/bs";
import { HiDotsHorizontal } from "react-icons/hi";
import AudioWavesIcon from "../../../../UI/AudioWavesIcon";

interface ClassNames {
  item_container: string;
  image_wrapper: string;
  image_hover: string;
  text_info: string;
  name_and_author: string;
  name: string;
  author: string;
  price_and_actions: string;
  price: string;
  buy_button: string;
  like_container: string;
  like_icon: string;
}

interface CarouselItemProps {
  carousel_type: "recommended" | "new";
  carousel_items: CarouselItem;
  index: number;
  hover_hidden: boolean;
}

const CarouselItem: FC<CarouselItemProps> = memo(
  ({ carousel_type, carousel_items, index, hover_hidden }) => {
    const formatted_price = carousel_items.price.toLocaleString("ru-RU");
    const new_item_class_name = `item${index}`;
    const class_names = useMemo<ClassNames>(() => {
      const base_class_names: ClassNames = {
        item_container: `${styles.item_container}`,
        image_wrapper: `${styles.image_wrapper}`,
        image_hover: `${styles.image_hover}`,
        text_info: `${styles.text_info}`,
        name_and_author: `${styles.name_and_author}`,
        name: `${styles.name}`,
        author: `${styles.author}`,
        price_and_actions: `${styles.price_and_actions}`,
        price: `${styles.price}`,
        buy_button: `${styles.buy_button}`,
        like_container: `${styles.like_container}`,
        like_icon: `${styles.like_icon}`,
      };

      switch (carousel_type) {
        case "recommended":
          break;
        case "new":
          base_class_names.item_container += ` ${styles.item_container_new} ${styles[new_item_class_name]}`;
          base_class_names.image_wrapper += ` ${styles.image_wrapper_new}`;
          base_class_names.image_hover += ` ${styles.image_hover_new}`;
          base_class_names.text_info += ` ${styles.text_info_new}`;
          base_class_names.name_and_author += ` ${styles.name_and_author_new}`;
          base_class_names.name += ` ${styles.name_new}`;
          base_class_names.author += ` ${styles.author_new}`;
          base_class_names.price_and_actions += ` ${styles.price_and_actions_new}`;
          base_class_names.price += ` ${styles.price_new}`;
          base_class_names.buy_button += ` ${styles.buy_button_new}`;
          base_class_names.like_container += ` ${styles.like_container_new}`;
          base_class_names.like_icon += ` ${styles.like_icon_new}`;
          break;
      }

      return base_class_names;
    }, [carousel_type]);

    return (
      <div className={`${class_names.item_container}`}>
        <div className={class_names.image_wrapper}>
          <img
            src={carousel_items.cover_url}
            alt="album cover"
            draggable={false}
          />
          {!hover_hidden && (
            <div className={class_names.image_hover}>
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
          )}
        </div>
        <div className={class_names.text_info}>
          <div className={class_names.name_and_author}>
            <p className={class_names.name}>{carousel_items.name}</p>
            <p className={class_names.author}>by {carousel_items.author}</p>
          </div>
          <div className={class_names.price_and_actions}>
            <p className={class_names.price}>{formatted_price} ₽</p>
            <div className={styles.action_buttons}>
              <div className={class_names.buy_button}>купить</div>
              <div className={class_names.like_container}>
                <FaRegHeart className={class_names.like_icon} />
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }
);

export default CarouselItem;
