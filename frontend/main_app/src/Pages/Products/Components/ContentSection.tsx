import styles from "./ContentSection.module.scss";
import underline_red from "../../../assets/svg/underline_red.svg";
import underline_coral from "../../../assets/svg/underline_coral.svg";
import underline_lilac from "../../../assets/svg/underline_lilac.svg";
import underline_green from "../../../assets/svg/underline_green.svg";
import { FC, useEffect, useRef, useState } from "react";
import Carousel from "./UI/Carousel";
import { carousel_items } from "./UI/content_dummies";
import MainContentProducts from "./MainContentProducts";
import { NavLink } from "react-router-dom";

interface ContentSectionProps {
  section_type: "beats" | "covers" | "songs" | "texts";
}

interface SectionProps {
  section_name: string;
  underline: string;
}

interface PopUpItem {
  name: string;
  link: string;
}

const ContentSection: FC<ContentSectionProps> = ({ section_type }) => {
  const [is_small_screen, set_is_small_screen] = useState(
    window.innerWidth <= 1024
  );
  const [pop_up_active, set_pop_up_active] = useState(false);
  const [pop_up_style, set_pop_up_style] = useState<React.CSSProperties>({});
  const [is_section_name_hovered, set_is_section_name_hovered] =
    useState(false);
  const [links_class_names, set_links_class_names] = useState({
    link_1: `${styles.pop_up_item}`,
    link_2: `${styles.pop_up_item}`,
    link_3: `${styles.pop_up_item}`,
  });
  const section_name_ref = useRef<HTMLDivElement>(null);

  let section_props: SectionProps = {
    section_name: section_type,
    underline: underline_red,
  };

  let pop_up_items: PopUpItem[] = [
    { name: "песен", link: "../songs" },
    { name: "битов", link: "../beats" },
    { name: "обложек", link: "../covers" },
    { name: "текстов", link: "../texts" },
  ];

  switch (section_type) {
    case "beats":
      section_props.section_name = "битов";
      section_props.underline = underline_coral;
      pop_up_items = [
        pop_up_items[1],
        pop_up_items[2],
        pop_up_items[3],
        pop_up_items[0],
      ];
      break;
    case "covers":
      section_props.section_name = "обложек";
      section_props.underline = underline_lilac;
      pop_up_items = [
        pop_up_items[2],
        pop_up_items[3],
        pop_up_items[0],
        pop_up_items[1],
      ];
      break;
    case "songs":
      section_props.section_name = "песен";
      break;
    case "texts":
      section_props.section_name = "текстов";
      section_props.underline = underline_green;
      pop_up_items = [
        pop_up_items[3],
        pop_up_items[0],
        pop_up_items[1],
        pop_up_items[2],
      ];
      break;
  }

  useEffect(() => {
    if (is_small_screen) {
      if (pop_up_active) {
        set_links_class_names({
          link_1: `${styles.pop_up_item} ${styles.item_1}`,
          link_2: `${styles.pop_up_item} ${styles.item_2}`,
          link_3: `${styles.pop_up_item} ${styles.item_3}`,
        });
      } else {
        set_links_class_names({
          link_1: `${styles.pop_up_item} ${styles.item_hidden}`,
          link_2: `${styles.pop_up_item} ${styles.item_hidden}`,
          link_3: `${styles.pop_up_item} ${styles.item_hidden}`,
        });
      }
    } else {
      if (is_section_name_hovered) {
        set_links_class_names({
          link_1: `${styles.pop_up_item} ${styles.item_1}`,
          link_2: `${styles.pop_up_item} ${styles.item_2}`,
          link_3: `${styles.pop_up_item} ${styles.item_3}`,
        });
      } else {
        set_links_class_names({
          link_1: `${styles.pop_up_item} ${styles.item_hidden}`,
          link_2: `${styles.pop_up_item} ${styles.item_hidden}`,
          link_3: `${styles.pop_up_item} ${styles.item_hidden}`,
        });
      }
    }
  }, [pop_up_active, is_section_name_hovered]);

  useEffect(() => {
    let timer: NodeJS.Timeout;
    if (is_small_screen) {
      if (!pop_up_active) {
        timer = setTimeout(() => {
          set_pop_up_style({ display: "none" });
        }, 200);
      } else {
        set_pop_up_style({ display: "flex" });
      }
    } else {
      if (!is_section_name_hovered) {
        timer = setTimeout(() => {
          set_pop_up_style({ display: "none" });
        }, 200);
      } else {
        set_pop_up_style({ display: "flex" });
      }
    }

    return () => {
      if (timer) {
        clearTimeout(timer);
      }
    };
  }, [pop_up_active, is_section_name_hovered]);

  useEffect(() => {
    const handle_click_outside_section_name = (e: MouseEvent) => {
      if (
        section_name_ref.current &&
        !section_name_ref.current.contains(e.target as Node)
      ) {
        set_pop_up_active(false);
      }
    };

    document.addEventListener("mousedown", handle_click_outside_section_name);

    return () => {
      document.removeEventListener(
        "mousedown",
        handle_click_outside_section_name
      );
    };
  }, []);

  const handle_section_name_click = () => {
    if (!is_small_screen) {
      return;
    }
    set_pop_up_active(!pop_up_active);
  };

  const handle_mouse_enter = () => {
    if (is_small_screen) {
      return;
    }
    set_is_section_name_hovered(true);
  };

  const handle_mouse_leave = () => {
    if (is_small_screen) {
      return;
    }
    set_is_section_name_hovered(false);
  };

  useEffect(() => {
    const handle_resize = () => {
      set_is_small_screen(window.innerWidth <= 1024);
    };

    window.addEventListener("resize", handle_resize);

    return () => {
      window.removeEventListener("resize", handle_resize);
    };
  }, []);

  return (
    <>
      <section className={styles.products_section}>
        <div className={styles.header}>
          <h1 className={styles.h1}>Библиотека</h1>
          <div
            className={styles.name_and_links}
            onMouseLeave={handle_mouse_leave}
          >
            <div
              className={styles.section_name}
              ref={section_name_ref}
              onClick={handle_section_name_click}
              onMouseEnter={handle_mouse_enter}
            >
              {section_props.section_name}
              <img
                src={section_props.underline}
                alt="underline"
                className={styles.underline}
              />
            </div>
            <div
              className={styles.pop_up_menu}
              style={pop_up_style}
            >
              <NavLink
                to={pop_up_items[1].link}
                className={links_class_names.link_1}
              >
                {pop_up_items[1].name}
              </NavLink>
              <NavLink
                to={pop_up_items[2].link}
                className={links_class_names.link_2}
              >
                {pop_up_items[2].name}
              </NavLink>
              <NavLink
                to={pop_up_items[3].link}
                className={links_class_names.link_3}
              >
                {pop_up_items[3].name}
              </NavLink>
            </div>
          </div>
        </div>
        {section_type !== "songs" ? (
          <div className={styles.template}>
            Секция {section_props.section_name}
          </div>
        ) : (
          <div className={styles.content}>
            <div className={styles.recommendations_block}>
              <Carousel
                carousel_type="recommended"
                carousel_items={carousel_items}
              />
            </div>
            <div className={styles.new_block}>
              <Carousel
                carousel_type="new"
                carousel_items={carousel_items}
              />
            </div>
            <div className={styles.main_content}>
              <h2 className={styles.h2}>Все песни</h2>
              <MainContentProducts />
            </div>
          </div>
        )}
      </section>
    </>
  );
};

export default ContentSection;
