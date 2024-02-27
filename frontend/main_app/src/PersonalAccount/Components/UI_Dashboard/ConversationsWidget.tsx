import { NavLink } from "react-router-dom";
import { ActiveSections, IConversationUnit } from "../../../types/types";
import styles from "./ConversationsWidget.module.scss";
import { FC } from "react";
import ConversationUnit from "./ConversationUnit";
import { useDispatch } from "react-redux";
import {
  set_sidebar_chevron_display,
  set_sidebar_collapsed,
  set_sidebar_title,
} from "../../../state/sidebar_actions_slice";

const mock_conversations: IConversationUnit[] = [
  {
    interlocutor_name: "Иван",
    message: "Привет, как дела?",
    message_author: "interlocutor",
    online_status: true,
    time_stamp: "10:49",
    unread_messages: 0,
  },
  {
    interlocutor_name: "Мария",
    message: "Все хорошо, спасибо",
    message_author: "user",
    online_status: false,
    time_stamp: "пн",
    unread_messages: 1,
  },
  {
    interlocutor_name: "Петр",
    message: "Как продвигается проект?",
    message_author: "interlocutor",
    online_status: true,
    time_stamp: "12:15",
    unread_messages: 0,
  },
  {
    interlocutor_name: "Ольга",
    message: "Все идет по плану",
    message_author: "user",
    online_status: false,
    time_stamp: "13:07",
    unread_messages: 2,
  },
  {
    interlocutor_name: "Сергей",
    message: "Есть ли какие-то новости?",
    message_author: "interlocutor",
    online_status: true,
    time_stamp: "вс",
    unread_messages: 0,
  },
  {
    interlocutor_name: "Анна",
    message: "Да, есть несколько",
    message_author: "user",
    online_status: false,
    time_stamp: "15:19",
    unread_messages: 1,
  },
  {
    interlocutor_name: "Дмитрий",
    message: "Что там с треком?",
    message_author: "interlocutor",
    online_status: true,
    time_stamp: "16:32",
    unread_messages: 0,
  },
  {
    interlocutor_name: "Екатерина",
    message: "Ничего особенного",
    message_author: "user",
    online_status: false,
    time_stamp: "17:21",
    unread_messages: 2,
  },
  {
    interlocutor_name: "Михаил",
    message: "Как настроение?",
    message_author: "interlocutor",
    online_status: true,
    time_stamp: "18:47",
    unread_messages: 0,
  },
  {
    interlocutor_name: "Юлия",
    message: "Все отлично",
    message_author: "user",
    online_status: false,
    time_stamp: "19:35",
    unread_messages: 1,
  },
];

const ConversationsWidget: FC = () => {
  const dispatch = useDispatch();
  const handle_tab_link_click = () => {
    dispatch(set_sidebar_collapsed(true));
    dispatch(set_sidebar_title("H.S"));
    dispatch(set_sidebar_chevron_display("none"));
  };
  return (
    <div className={styles.conversations_widget}>
      <div className={styles.top_bar}>
        <p className={styles.header}>
          Беседы <span>(5)</span>
        </p>
        <NavLink
          to="../conversations"
          className={styles.show_all}
          onClick={handle_tab_link_click}
        >
          показать все
        </NavLink>
      </div>
      <hr className={styles.border_h} />
      <div className={styles.main_content}>
        {mock_conversations.map((conversation_unit, idx) => {
          return (
            <ConversationUnit
              key={idx}
              interlocutor_name={conversation_unit.interlocutor_name}
              message={conversation_unit.message}
              message_author={conversation_unit.message_author}
              online_status={conversation_unit.online_status}
              time_stamp={conversation_unit.time_stamp}
              unread_messages={conversation_unit.unread_messages}
            />
          );
        })}
        <hr className={styles.border_v} />
      </div>
      <hr className={styles.border_h} />
    </div>
  );
};

export default ConversationsWidget;
