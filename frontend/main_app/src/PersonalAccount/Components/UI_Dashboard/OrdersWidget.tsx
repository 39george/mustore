import styles from "./OrdersWidget.module.scss";
import { FC } from "react";
import { IoChevronDownOutline } from "react-icons/io5";
import { IOrderUnit } from "../../../types/types";
import OrderUnit from "./OrderUnit";

const mock_orders: IOrderUnit[] = [
  {
    consumer: "Brian",
    price: "10 000₽",
    deliver_to: "1д 5ч",
    status: "в работе",
  },
  {
    consumer: "Tolya",
    price: "3 000₽",
    deliver_to: "3д 12ч",
    status: "в работе",
  },
  {
    consumer: "Peter",
    price: "17 000₽",
    deliver_to: "-- --",
    status: "доставлен",
  },
];

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
      {mock_orders.map((order, idx) => {
        return (
          <OrderUnit
            key={idx}
            consumer={order.consumer}
            price={order.price}
            deliver_to={order.deliver_to}
            status={order.status}
          />
        );
      })}
    </div>
  );
};

export default OrdersWidget;
