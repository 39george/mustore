import styles from "./Carousel.module.scss";
import React, { FC, useEffect, useMemo, useRef, useState } from "react";
import { GoChevronDown } from "react-icons/go";
import CarouselItem from "./CarouselItem";
import { carousel_items_rec } from "./content_dummies";

interface CarouselProps {
  carousel_type: "recommendations" | "new";
}

interface ClassNames {
  carousel_container: string;
  carousel_inner: string;
}

const Carousel: FC<CarouselProps> = ({ carousel_type }) => {
  const [current_index, set_current_index] = useState(0);
  const [container_width, set_container_widht] = useState(0);
  const [items_per_slide, set_items_per_slide] = useState(1);
  const MIN_ITEMS_PER_SLIDE = 1;
  const MAX_ITEMS_PER_SLIDE = 6;
  const MAX_INDEX =
    carousel_items_rec.length -
    (carousel_items_rec.length % items_per_slide || items_per_slide);
  const ITEM_WIDTH = 172;
  const RIGHT_MARGIN = 28;
  const [is_next_hovered, set_is_next_hovered] = useState(false);
  const carousel_ref = useRef<HTMLDivElement>(null);
  const class_names = useMemo<ClassNames>(() => {
    const base_class_names: ClassNames = {
      carousel_container: `${styles.carousel_container}`,
      carousel_inner: `${styles.carousel_inner}`,
    };

    switch (carousel_type) {
      case "recommendations":
        break;
      case "new":
        base_class_names.carousel_container += `${styles.carousel_container_new}`;
        base_class_names.carousel_inner += `${styles.carousel_inner_new}`;
        break;
    }

    return base_class_names;
  }, [carousel_type]);

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
  };

  const handle_prev = () => {
    set_current_index((prev_index) =>
      Math.max(prev_index - items_per_slide, 0)
    );
  };

  useEffect(() => {
    current_index === MAX_INDEX ? set_is_next_hovered(false) : "";
  }, [current_index]);

  // Calculating translation amount
  const get_translation_amount = () => {
    const total_content_width = carousel_items_rec.length * ITEM_WIDTH;
    const max_translation =
      total_content_width - container_width + RIGHT_MARGIN;
    let translation_for_current_index = current_index * ITEM_WIDTH;
    let translation = Math.min(translation_for_current_index, max_translation);

    return -translation;
  };

  // Rendering component
  return (
    <div
      className={class_names.carousel_container}
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
          className={class_names.carousel_inner}
          style={{
            transform: `translateX(${get_translation_amount()}px)`,
          }}
        >
          {carousel_items_rec.map((item) => {
            return (
              <React.Fragment key={item.id}>
                <CarouselItem
                  cover_url={item.cover_url}
                  name={item.name}
                  author={item.author}
                  price={item.price}
                  likes={item.likes}
                  type={item.type}
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
          onMouseEnter={() => set_is_next_hovered(true)}
          onMouseLeave={() => set_is_next_hovered(false)}
        >
          <GoChevronDown className={styles.next_chevron} />
        </div>
      )}

      <div className={styles.show_all}>показать все...</div>
    </div>
  );
};

export default Carousel;
