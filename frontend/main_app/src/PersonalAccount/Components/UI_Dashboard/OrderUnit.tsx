import { ActiveTabsAccountCreator, IOrderUnit } from "../../../types/types";
import styles from "./OrderUnit.module.scss";
import { FC, useEffect, useState } from "react";
import order_photo from "../../../assets/HomePage/album_covers/ablum_cover_9.png";
import avatar from "../../../assets/HomePage/author_2.png";
import { NavLink } from "react-router-dom";
import { useDispatch } from "react-redux";
import { set_active_tab_account_creator } from "../../../state/active_tab_account_creator_slice";

const OrderUnit: FC<IOrderUnit> = ({ consumer, deliver_to, price, status }) => {
  const [status_class_names, set_status_class_names] = useState(
    `${styles.info_status}`
  );
  const dispatch = useDispatch();

  const handle_link_click = (active_tab: ActiveTabsAccountCreator) => {
    dispatch(set_active_tab_account_creator(active_tab));
  };

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
      <div className={styles.service_image_wrapper}>
        <img
          src={order_photo}
          alt="order photo"
        />
        <div className={styles.consumer_image_wrapper}>
          <img
            src={avatar}
            alt="consumer's avatar"
          />
        </div>
      </div>
      <div className={styles.order_info_container}>
        <div className={styles.order_main_info}>
          <div className={styles.order_info}>
            <p className={styles.info_type}>Покупатель</p>
            <p className={styles.info_content}>{consumer}</p>
          </div>
          <p className={styles.consumer_name}>
            {consumer}
            <span className={styles.online_status}></span>
          </p>
          <hr className={styles.divider} />
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
        onClick={() => handle_link_click("orders")}
      >
        к заказу
      </NavLink>
    </div>
  );
};

export default OrderUnit;
