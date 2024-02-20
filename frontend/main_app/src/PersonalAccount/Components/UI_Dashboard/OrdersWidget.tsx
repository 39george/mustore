import styles from "./OrdersWidget.module.scss";
import { FC } from "react";
import { IoChevronDownOutline } from "react-icons/io5";

const OrdersWidget: FC = () => {
  return (
    <div className={styles.orders_widget}>
      <div className={styles.order_type}>
        <div className={styles.name_and_price}>
          <p className={styles.type_name}>Текущие заказы&nbsp;</p>
          <p className={styles.sum_price}> - 3 (30 000₽)</p>
        </div>
        <div className={styles.type_selector}>
          <p className={styles.current_select}>
            текущие заказы <span>(3)</span>
          </p>
          <IoChevronDownOutline className={styles.chevron} />
        </div>
      </div>
    </div>
  );
};

export default OrdersWidget;
