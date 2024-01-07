import styles from "./Carousel.module.scss";
import React, { FC, useEffect, useMemo, useRef, useState } from "react";
import { GoChevronDown } from "react-icons/go";
import CarouselItem from "./CarouselItem";
import { CarouselProps } from "../../../../types/types";

interface ClassNames {
  carousel_container: string;
  carousel_inner: string;
}

const Carousel: FC<CarouselProps> = ({ carousel_type, carousel_items }) => {
  const [current_index, set_current_index] = useState(0);
  const [container_width, set_container_widht] = useState(0);
  const [items_per_slide, set_items_per_slide] = useState(1);
  const config = useMemo(() => {
    let config = {
      MIN_ITEMS_PER_SLIDE: 1,
      MAX_ITEMS_PER_SLIDE: 6,
      ITEM_WIDTH: 172,
      TOTAL_CONTENT_WIDTH: 0,
      MAX_INDEX:
        carousel_items.length -
        (carousel_items.length % items_per_slide || items_per_slide),
      RIGHT_MARGIN: 28,
    };

    switch (carousel_type) {
      case "recommended":
        config.TOTAL_CONTENT_WIDTH = carousel_items.length * config.ITEM_WIDTH;
        break;
      case "new":
        config.MAX_ITEMS_PER_SLIDE = 3;
        config.ITEM_WIDTH = 340;
        config.TOTAL_CONTENT_WIDTH =
          Math.ceil(carousel_items.length / 3) * config.ITEM_WIDTH - 36;
        config.MAX_INDEX =
          Math.ceil(carousel_items.length / 3) -
          (carousel_items.length % items_per_slide || items_per_slide);
    }

    return config;
  }, [carousel_type, items_per_slide, carousel_items.length]);
  // const MAX_INDEX =
  //   carousel_items.length -
  //   (carousel_items.length % items_per_slide || items_per_slide);
  const [is_next_hovered, set_is_next_hovered] = useState(false);
  const carousel_ref = useRef<HTMLDivElement>(null);
  const class_names = useMemo<ClassNames>(() => {
    const base_class_names: ClassNames = {
      carousel_container: `${styles.carousel_container}`,
      carousel_inner: `${styles.carousel_inner}`,
    };

    switch (carousel_type) {
      case "recommended":
        break;
      case "new":
        base_class_names.carousel_container += ` ${styles.carousel_container_new}`;
        base_class_names.carousel_inner += ` ${styles.carousel_inner_new}`;
        break;
    }

    return base_class_names;
  }, [carousel_type]);
  const h2_name = useMemo(() => {
    switch (carousel_type) {
      case "recommended":
        return "Рекомендации";
      case "new":
        return "Новинки";
    }
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
    const new_items_per_slide = Math.floor(container_width / config.ITEM_WIDTH);

    const clamped_items_per_slide = Math.max(
      config.MIN_ITEMS_PER_SLIDE,
      Math.min(new_items_per_slide, config.MAX_ITEMS_PER_SLIDE)
    );

    set_items_per_slide(clamped_items_per_slide);
  }, [container_width]);

  // Handling next/prev buttons
  const handle_next = () => {
    set_current_index((prev_index) =>
      Math.min(prev_index + items_per_slide, config.MAX_INDEX)
    );
  };

  const handle_prev = () => {
    set_current_index((prev_index) =>
      Math.max(prev_index - items_per_slide, 0)
    );
  };

  useEffect(() => {
    current_index === config.MAX_INDEX ? set_is_next_hovered(false) : "";
  }, [current_index]);

  // Calculating translation amount
  const get_translation_amount = () => {
    const max_translation =
      config.TOTAL_CONTENT_WIDTH - container_width + config.RIGHT_MARGIN;
    let translation_for_current_index = current_index * config.ITEM_WIDTH;
    let translation = Math.min(translation_for_current_index, max_translation);

    return -translation;
  };

  // Rendering component
  return (
    <div
      className={class_names.carousel_container}
      ref={carousel_ref}
    >
      <h2 className={styles.h2}>{h2_name}</h2>
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
          style={
            {
              "--num-columns": `${Math.ceil(carousel_items.length / 3)}`,
              transform: `translateX(${get_translation_amount()}px)`,
            } as React.CSSProperties
          }
        >
          {carousel_items.map((item) => {
            return (
              <React.Fragment key={item.id}>
                <CarouselItem
                  carousel_items={item}
                  carousel_type={carousel_type}
                />
              </React.Fragment>
            );
          })}
        </div>
      </div>
      {current_index !== config.MAX_INDEX && (
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
