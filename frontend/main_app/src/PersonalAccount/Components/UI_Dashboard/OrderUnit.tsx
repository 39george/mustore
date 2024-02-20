import { IOrderUnit } from "../../../types/types";
import styles from "./OrderUnit.module.scss";
import { FC, useEffect, useState } from "react";
import order_photo from "../../../assets/HomePage/album_covers/ablum_cover_9.png";
import { NavLink } from "react-router-dom";

const OrderUnit: FC<IOrderUnit> = ({ consumer, deliver_to, price, status }) => {
  const [status_class_names, set_status_class_names] = useState(
    `${styles.info_status}`
  );

  useEffect(() => {
    switch (status) {
      case "в работе":
        set_status_class_names(`${styles.info_status} ${styles.in_progress}`);
        break;
      case "доставлен":
        set_status_class_names(`${styles.info_status} ${styles.delivered}`);
    }
  }, [status]);
  return (
    <div className={styles.order_unit}>
      <div className={styles.image_wrapper}>
        <img
          src={order_photo}
          alt="order photo"
        />
      </div>
      <div className={styles.order_info_container}>
        <div className={styles.order_main_info}>
          <div className={styles.order_info}>
            <p className={styles.info_type}>Покупатель</p>
            <p className={styles.info_content}>{consumer}</p>
          </div>
          <div className={styles.order_info}>
            <p className={styles.info_type}>Цена</p>
            <p className={styles.info_content}>{price}</p>
          </div>
          <div className={styles.order_info}>
            <p className={styles.info_type}>Время до отдачи</p>
            <p className={styles.info_content}>{deliver_to}</p>
          </div>
        </div>
        <div className={styles.order_info}>
          <p className={styles.info_type}>Статус</p>
          <div className={status_class_names}>{status}</div>
        </div>
      </div>
      <NavLink
        to="../orders"
        className={styles.to_order_link}
      >
        к заказу
      </NavLink>
    </div>
  );
};

export default OrderUnit;
