import styles from "./Dashboard.module.scss";
import { NavLink } from "react-router-dom";
import MetainfoWidget from "../Components/UI_Dashboard/MetaInfoWidget";
import WelcomeWidget from "../Components/UI_Dashboard/WelcomeWidget";
import { FC } from "react";
import ConversationsWidget from "../Components/UI_Dashboard/ConversationsWidget";
import OrdersWidget from "../Components/UI_Dashboard/OrdersWidget";

const Dashboard: FC = () => {
  return (
    <div className={styles.dashboard}>
      <div className={styles.meta_and_welcome}>
        <MetainfoWidget />
        <div className={styles.welcome}>
          <h2 className={styles.h2}>
            Добро пожаловать, <span>Alena NAI</span>!
          </h2>
          <WelcomeWidget />
          <h2 className={`${styles.h2} ${styles.h2_workbench}`}>
            Ваше рабочее пространство
          </h2>
          <div className={styles.workbench}>
            <NavLink
              to="../products"
              className={styles.workbench_widget}
            >
              <p className={styles.workbench_widget_text}>
                Управление товарами
              </p>
            </NavLink>
            <NavLink
              to="../services"
              className={styles.workbench_widget}
            >
              <p className={styles.workbench_widget_text}>
                Управление услугами
              </p>
            </NavLink>
          </div>
        </div>
      </div>
      <h2 className={`${styles.h2} ${styles.h2_conversations_and_orders}`}>
        Беседы и заказы
      </h2>
      <div className={styles.conversations_and_orders}>
        <ConversationsWidget />
        <OrdersWidget />
      </div>
    </div>
  );
};

export default Dashboard;
