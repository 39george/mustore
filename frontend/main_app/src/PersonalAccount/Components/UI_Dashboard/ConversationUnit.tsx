import styles from "./ConversationUnit.module.scss";
import { FC } from "react";
import { IConversationUnit } from "../../../types/types";
import avatar from "../../../assets/HomePage/author_2.png";

const ConversationUnit: FC<IConversationUnit> = ({
  interlocutor_name,
  message,
  message_author,
  online_status,
  time_stamp,
  unread_messages,
}) => {
  return (
    <div className={styles.conversation_unit}>
      <div className={styles.image_wrapper}>
        <img
          src={avatar}
          alt="avatar"
        />
        <span
          className={`${styles.online_status} ${
            online_status ? styles.online : styles.offline
          }`}
        ></span>
      </div>
      <div className={styles.interlocutor_info}>
        <p className={styles.interlocutor_name}>{interlocutor_name}</p>
        <p className={styles.message}>
          {message_author === "interlocutor" ? "" : "Я: "}
          {message}
        </p>
      </div>
      <div className={styles.additional_info}>
        <p className={styles.time_stamp}>{time_stamp}</p>
        {unread_messages !== 0 && (
          <div className={styles.unread_messages}>{unread_messages}</div>
        )}
      </div>
    </div>
  );
};

export default ConversationUnit;
