import { NavLink } from "react-router-dom";
import { IServiceItem } from "../../../../types/types";
import styles from "./ServiceItem.module.scss";
import { FC } from "react";
import usePageNavigation from "../../../../hooks/usePageNavigation";

const ServiceItem: FC<IServiceItem> = ({ icon, title, description }) => {
  const handle_page_navigation = usePageNavigation();
  return (
    <div className={styles.service}>
      <div className={styles.icon_wrapper}>
        <img
          src={icon}
          alt="service icon"
          className={styles.service_icon}
        />
      </div>
      <div className={styles.service_text}>
        <h3 className={styles.h3}>{title}</h3>
        <p className={styles.description}>{description}</p>
        <NavLink
          to="services"
          className={styles.link}
          onClick={() => handle_page_navigation("services")}
        >
          найти профессионала
        </NavLink>
      </div>
    </div>
  );
};

export default ServiceItem;
