import MetainfoWidget from "../Components/UI/MetaInfoWidget";
import styles from "./Dashboard.module.scss";
import { FC } from "react";

const Dashboard: FC = () => {
  return (
    <div className={styles.dashboard}>
      <MetainfoWidget />
    </div>
  );
};

export default Dashboard;
