import styles from "./ConversationUnit.module.scss";
import { FC, useState } from "react";
import { IConversationUnit } from "../../../types/types";
import avatar from "../../../assets/HomePage/author_2.png";
import { BsThreeDots } from "react-icons/bs";

const ConversationUnit: FC<IConversationUnit> = ({
  interlocutor_name,
  message,
  message_author,
  online_status,
  time_stamp,
  unread_messages,
}) => {
  const [hovered, set_hovered] = useState(false);

  return (
    <div
      className={styles.conversation_unit}
      onMouseEnter={() => set_hovered(!hovered)}
      onMouseLeave={() => set_hovered(!hovered)}
    >
      <div className={styles.meta_info}>
        <div className={styles.image_wrapper}>
          <img
            src={avatar}
            alt="avatar"
          />
        </div>
        <div className={styles.interlocutor_info}>
          <p className={styles.interlocutor_name}>{interlocutor_name}</p>
          <p className={styles.message}>
            {message_author === "interlocutor" ? "" : "Я: "}
            {message}
          </p>
        </div>
        <span
          className={`${styles.online_status} ${
            online_status ? styles.online : styles.offline
          }`}
        ></span>
      </div>
      <div className={styles.additional_info}>
        {!hovered ? (
          <p className={styles.time_stamp}>{time_stamp}</p>
        ) : (
          <BsThreeDots className={styles.message_options} />
        )}

        {unread_messages !== 0 ? (
          <div className={styles.unread_messages}>{unread_messages}</div>
        ) : (
          <span className={styles.filler}></span>
        )}
      </div>
    </div>
  );
};

export default ConversationUnit;
