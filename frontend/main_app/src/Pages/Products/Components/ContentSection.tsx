import styles from "./ContentSection.module.scss";
import underline_red from "../../../assets/svg/underline_red.svg";
import underline_coral from "../../../assets/svg/underline_coral.svg";
import underline_lilac from "../../../assets/svg/underline_lilac.svg";
import underline_green from "../../../assets/svg/underline_green.svg";
import { FC, useState } from "react";
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
  const [links_class_names, set_links_class_names] = useState({
    link_1: `${styles.pop_up_item}`,
    link_2: `${styles.pop_up_item}`,
    link_3: `${styles.pop_up_item}`,
  });

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

  const handle_mouse_enter = () => {
    set_links_class_names({
      link_1: `${styles.pop_up_item} ${styles.item_1}`,
      link_2: `${styles.pop_up_item} ${styles.item_2}`,
      link_3: `${styles.pop_up_item} ${styles.item_3}`,
    });
  };

  const handle_mouse_leave = () => {
    set_links_class_names({
      link_1: `${styles.pop_up_item} ${styles.item_hidden}`,
      link_2: `${styles.pop_up_item} ${styles.item_hidden}`,
      link_3: `${styles.pop_up_item} ${styles.item_hidden}`,
    });
  };

  return (
    <section className={styles.products_section}>
      <div className={styles.header}>
        <h1 className={styles.h1}>Библиотека</h1>
        <div
          className={styles.name_and_links}
          onMouseLeave={handle_mouse_leave}
        >
          <div
            className={styles.section_name}
            onMouseEnter={handle_mouse_enter}
          >
            {section_props.section_name}
            <img
              src={section_props.underline}
              alt="underline"
              className={styles.underline}
            />
          </div>
          <div className={styles.pop_up_menu}>
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
  );
};

export default ContentSection;
