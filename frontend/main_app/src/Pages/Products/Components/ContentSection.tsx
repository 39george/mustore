import styles from "./ContentSection.module.scss";
import { GoChevronDown } from "react-icons/go";
import underline_red from "../../../assets/svg/underline_red.svg";
import underline_coral from "../../../assets/svg/underline_coral.svg";
import underline_lilac from "../../../assets/svg/underline_lilac.svg";
import underline_green from "../../../assets/svg/underline_green.svg";
import stars from "../../../assets/svg/recommendations.svg";
import fire from "../../../assets/svg/fire.svg";
import { FC } from "react";
import RecCarousel from "./UI/RecCarousel";

interface ContentSectionProps {
  section_type: "beats" | "covers" | "songs" | "texts";
}

interface SectionTypeProps {
  section_name: string;
  underline: string;
}

const ContentSection: FC<ContentSectionProps> = ({ section_type }) => {
  let section_props: SectionTypeProps = {
    section_name: section_type,
    underline: underline_red,
  };

  switch (section_type) {
    case "beats":
      section_props.section_name = "битов";
      section_props.underline = underline_coral;
      break;
    case "covers":
      section_props.section_name = "обложек";
      section_props.underline = underline_lilac;
      break;
    case "songs":
      section_props.section_name = "песен";
      section_props.underline = underline_red;
      break;
    case "texts":
      section_props.section_name = "текстов";
      section_props.underline = underline_green;
      break;
  }

  return (
    <section className={styles.content_section}>
      <div className={styles.header}>
        <h1 className={styles.h1}>
          Библиотека{" "}
          <span>
            {section_props.section_name}
            <img
              src={section_props.underline}
              alt="underline"
              className={styles.underline}
            />
          </span>
        </h1>
        <GoChevronDown className={styles.chevron} />
      </div>
      {section_type !== "songs" ? (
        <div className={styles.template}>
          Секция {section_props.section_name}
        </div>
      ) : (
        <div className={styles.main_content}>
          <div className={styles.recommendations_block}>
            <div className={styles.title}>
              <h2 className={styles.h2}>
                Рекомендации harmony<span>.</span>sphere
              </h2>
              <img
                src={stars}
                alt="stars"
                className={styles.title_icon}
              />
            </div>
            <RecCarousel />
          </div>
        </div>
      )}
    </section>
  );
};

export default ContentSection;
