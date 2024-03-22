import styles from "./Dashboard.module.scss";
import { NavLink } from "react-router-dom";
import MetainfoWidget from "../Components/UI_Dashboard/MetaInfoWidget";
import WelcomeWidget from "../Components/UI_Dashboard/WelcomeWidget";
import { FC } from "react";
import ConversationsWidget from "../Components/UI_Dashboard/ConversationsWidget";
import OrdersWidget from "../Components/UI_Dashboard/OrdersWidget";
import { useDispatch, useSelector } from "react-redux";
import { RootState } from "../../state/store";
import { ActiveTabsAccountCreator } from "../../types/types";
import { set_active_tab_account_creator } from "../../state/active_tab_account_creator_slice";

interface DashboardProps {
  username: string;
  user_role: string;
  avatar: string;
}

const Dashboard: FC<DashboardProps> = ({ username, user_role, avatar }) => {
  const sidebar_collapsed = useSelector(
    (state: RootState) => state.sidebar_actions.sidebar_collapsed
  );
  const dispatch = useDispatch();

  const handle_link_click = (active_tab: ActiveTabsAccountCreator) => {
    dispatch(set_active_tab_account_creator(active_tab));
  };

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
            Добро пожаловать, <br className={styles.br} />
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
              onClick={() => handle_link_click("products")}
            >
              <p className={styles.workbench_widget_text}>
                Управление товарами
              </p>
            </NavLink>
            <NavLink
              to="../services"
              className={styles.workbench_widget}
              onClick={() => handle_link_click("services")}
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
