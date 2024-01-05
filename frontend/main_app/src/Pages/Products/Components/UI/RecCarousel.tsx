import styles from "./RecCarousel.module.scss";
import React, { FC, useEffect, useRef, useState } from "react";
import cover_1 from "../../../../assets/HomePage/album_covers/ablum_cover_1.png";
import cover_2 from "../../../../assets/HomePage/album_covers/ablum_cover_2.png";
import cover_3 from "../../../../assets/HomePage/album_covers/ablum_cover_3.png";
import cover_4 from "../../../../assets/HomePage/album_covers/ablum_cover_4.png";
import cover_5 from "../../../../assets/HomePage/album_covers/ablum_cover_5.png";
import cover_6 from "../../../../assets/HomePage/album_covers/ablum_cover_6.png";
import cover_7 from "../../../../assets/HomePage/album_covers/ablum_cover_7.png";
import cover_8 from "../../../../assets/HomePage/album_covers/ablum_cover_8.png";
import { GoChevronDown } from "react-icons/go";
import { CarouselItem } from "../../../../types/types";
import RecCarouselItem from "./RecCarouselItem";

const carousel_items: CarouselItem[] = [
  {
    id: 1,
    cover_url: cover_1,
    name: "Будь рядом",
    author: "ALENA NAI",
    price: 100_000,
    likes: 49,
  },
  {
    id: 2,
    cover_url: cover_2,
    name: "Закат",
    author: "KartenMusic",
    price: 70_000,
    likes: 35,
  },
  {
    id: 3,
    cover_url: cover_3,
    name: "Ночные часы",
    author: "Camely",
    price: 90_000,
    likes: 32,
  },
  {
    id: 4,
    cover_url: cover_4,
    name: "Забуду тебя",
    author: "Atires",
    price: 60_000,
    likes: 29,
  },
  {
    id: 5,
    cover_url: cover_5,
    name: "В окне",
    author: "SIBERD",
    price: 30_000,
    likes: 13,
  },
  {
    id: 6,
    cover_url: cover_6,
    name: "Вечером одна",
    author: "Ababbul",
    price: 56_000,
    likes: 21,
  },
  {
    id: 7,
    cover_url: cover_7,
    name: "Будь рядом",
    author: "ALENA NAI",
    price: 100_000,
    likes: 49,
  },
  {
    id: 8,
    cover_url: cover_8,
    name: "Закат",
    author: "KartenMusic",
    price: 70_000,
    likes: 35,
  },
  {
    id: 9,
    cover_url: cover_3,
    name: "Ночные часы",
    author: "Camely",
    price: 90_000,
    likes: 32,
  },
  {
    id: 10,
    cover_url: cover_4,
    name: "Забуду тебя",
    author: "Atires",
    price: 60_000,
    likes: 29,
  },
  {
    id: 11,
    cover_url: cover_5,
    name: "В окне",
    author: "SIBERD",
    price: 30_000,
    likes: 13,
  },
  {
    id: 12,
    cover_url: cover_6,
    name: "Вечером одна",
    author: "Ababbul",
    price: 56_000,
    likes: 21,
  },
  {
    id: 13,
    cover_url: cover_1,
    name: "Вечером одна",
    author: "Ababbul",
    price: 56_000,
    likes: 21,
  },
  {
    id: 14,
    cover_url: cover_7,
    name: "Будь рядом",
    author: "ALENA NAI",
    price: 100_000,
    likes: 49,
  },
  {
    id: 15,
    cover_url: cover_8,
    name: "Закат",
    author: "KartenMusic",
    price: 70_000,
    likes: 35,
  },
];

const RecCarousel: FC = () => {
  const [current_index, set_current_index] = useState(0);
  const [container_width, set_container_widht] = useState(0);
  const [items_per_slide, set_items_per_slide] = useState(2);
  const MIN_ITEMS_PER_SLIDE = 1;
  const MAX_ITEMS_PER_SLIDE = 6;
  const MAX_INDEX =
    carousel_items.length -
    (carousel_items.length % items_per_slide || items_per_slide);
  const ITEM_WIDTH = 172;
  const RIGHT_MARGIN = 28;
  const [is_next_hovered, set_is_next_hovered] = useState(false);
  const carousel_ref = useRef<HTMLDivElement>(null);

  const handle_next_button_hover = (hovered: boolean) => {
    set_is_next_hovered(hovered);
  };

  // Updating container_width
  const update_container_width = () => {
    set_container_widht(carousel_ref.current?.offsetWidth || 0);
  };

  useEffect(() => {
    update_container_width();

    const handle_resize = () => {
      update_container_width();
    };

    window.addEventListener("resize", handle_resize);

    return () => {
      window.removeEventListener("resize", handle_resize);
    };
  }, []);

  // Calculating new items per slide
  useEffect(() => {
    const new_items_per_slide = Math.floor(container_width / ITEM_WIDTH);

    const clamped_items_per_slide = Math.max(
      MIN_ITEMS_PER_SLIDE,
      Math.min(new_items_per_slide, MAX_ITEMS_PER_SLIDE)
    );

    set_items_per_slide(clamped_items_per_slide);
  }, [container_width]);

  // Handling next/prev buttons
  const handle_next = () => {
    set_current_index((prev_index) =>
      Math.min(prev_index + items_per_slide, MAX_INDEX)
    );

    if (current_index === MAX_INDEX - current_index) {
      console.log("here");
      set_is_next_hovered(false);
    }
  };

  const handle_prev = () => {
    set_current_index((prev_index) =>
      Math.max(prev_index - items_per_slide, 0)
    );
  };

  // Calculating translation amount
  const get_translation_amount = () => {
    const total_content_width = carousel_items.length * ITEM_WIDTH;
    const max_translation =
      total_content_width - container_width + RIGHT_MARGIN;
    let translation_for_current_index = current_index * ITEM_WIDTH;
    let translation = Math.min(translation_for_current_index, max_translation);

    return -translation;
  };

  // Rendering component
  return (
    <div
      className={styles.carousel_container}
      ref={carousel_ref}
    >
      {current_index !== 0 && (
        <div
          className={styles.prev_button}
          onClick={handle_prev}
        >
          <GoChevronDown className={styles.prev_chevron} />
        </div>
      )}
      <div
        className={styles.wrapper}
        style={{ transform: `${is_next_hovered ? "translateX(-20px)" : ""}` }}
      >
        <div
          className={styles.carousel_inner}
          style={{
            transform: `translateX(${get_translation_amount()}px)`,
          }}
        >
          {carousel_items.map((item) => {
            return (
              <React.Fragment key={item.id}>
                <RecCarouselItem
                  cover_url={item.cover_url}
                  name={item.name}
                  author={item.author}
                  price={item.price}
                  likes={item.likes}
                />
              </React.Fragment>
            );
          })}
        </div>
      </div>
      {current_index !== MAX_INDEX && (
        <div
          className={styles.next_button}
          onClick={handle_next}
          onMouseEnter={() => handle_next_button_hover(true)}
          onMouseLeave={() => handle_next_button_hover(false)}
        >
          <GoChevronDown className={styles.next_chevron} />
        </div>
      )}

      <div className={styles.show_all}>показать все...</div>
    </div>
  );
};

export default RecCarousel;
