import styles from "./Dashboard.module.scss";
import { NavLink } from "react-router-dom";
import MetainfoWidget from "../Components/UI_Dashboard/MetaInfoWidget";
import WelcomeWidget from "../Components/UI_Dashboard/WelcomeWidget";
import { FC, useEffect, useState } from "react";
import ConversationsWidget from "../Components/UI_Dashboard/ConversationsWidget";
import OrdersWidget from "../Components/UI_Dashboard/OrdersWidget";
import { useSelector } from "react-redux";
import { RootState } from "../../state/store";

const Dashboard: FC = () => {
  const sidebar_collapsed = useSelector(
    (state: RootState) => state.sidebar_actions.sidebar_collapsed
  );
  const username = useSelector(
    (state: RootState) => state.username_avatar.username
  );
  const avatar = useSelector(
    (state: RootState) => state.username_avatar.avatar
  );
  const [user_role, set_user_role] = useState("...");
  const user_permissions = useSelector(
    (state: RootState) => state.user_permissions
  );

  useEffect(() => {
    const index_creator = user_permissions.permissions.findIndex(
      (obj) => obj.name === "creator"
    );
    if (user_permissions.permissions[index_creator]?.name === "creator") {
      set_user_role("Автор");
    }
  }, [user_permissions]);

  return (
    <div className={styles.dashboard}>
      <div
        className={`${styles.meta_and_welcome} ${
          sidebar_collapsed && styles.margin_sidebar_collapsed
        }`}
      >
        <MetainfoWidget
          username={username}
          user_role={user_role}
          avatar={avatar}
        />
        <div className={styles.welcome}>
          <h2 className={styles.h2}>
            Добро пожаловать, <br />
            <span>{username}</span>!
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
      <h2
        className={`${styles.h2} ${styles.h2_conversations_and_orders} ${
          sidebar_collapsed && styles.h2_conversations_and_orders_margin
        }`}
      >
        Беседы и заказы
      </h2>
      <div
        className={`${styles.conversations_and_orders} ${
          sidebar_collapsed && styles.margin_sidebar_collapsed
        }`}
      >
        <ConversationsWidget />
        <OrdersWidget />
      </div>
    </div>
  );
};

export default Dashboard;
