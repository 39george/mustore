import styles from "./WelcomeWidget.module.scss";
import { FC } from "react";
import { IoChevronForwardOutline } from "react-icons/io5";

const billboard_messages: string[] = [
  "Гайд по личному кабинету",
  "Правила сообщества",
  "Новости сообщества",
];

const WelcomeWidget: FC = () => {
  return (
    <div className={styles.billboard}>
      {billboard_messages.map((message, idx) => {
        return (
          <div
            className={styles.billboard_unit}
            key={idx}
          >
            <p className={styles.billboard_message}>{message}</p>
            <IoChevronForwardOutline className={styles.chevron} />
          </div>
        );
      })}
    </div>
  );
};

export default WelcomeWidget;
