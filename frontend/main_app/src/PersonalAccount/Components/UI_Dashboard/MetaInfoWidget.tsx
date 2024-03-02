import styles from "./MetaInfoWidget.module.scss";
import { FC, useEffect, useState } from "react";
import { FaStar } from "react-icons/fa6";

const month = new Date().getMonth();

interface MetaInfoWidgetProps {
  username: string;
  user_role: string;
  avatar: string;
}

const MetainfoWidget: FC<MetaInfoWidgetProps> = ({
  username,
  user_role,
  avatar,
}) => {
  const [current_month, set_current_month] = useState("");

  useEffect(() => {
    switch (month) {
      case 0:
        set_current_month("Январе");
        break;
      case 1:
        set_current_month("Феврале");
        break;
      case 2:
        set_current_month("Марте");
        break;
      case 3:
        set_current_month("Апреле");
        break;
      case 4:
        set_current_month("Мае");
        break;
      case 5:
        set_current_month("Июне");
        break;
      case 6:
        set_current_month("Июле");
        break;
      case 7:
        set_current_month("Августе");
        break;
      case 8:
        set_current_month("Сентябре");
        break;
      case 9:
        set_current_month("Октябре");
        break;
      case 10:
        set_current_month("Ноябре");
        break;
      case 11:
        set_current_month("Декабре");
        break;
    }
  }, [month]);

  return (
    <div className={styles.metainfo_widget}>
      <div className={styles.main_info}>
        <div className={styles.header}>
          <div className={styles.image_wrapper}>
            <img
              src={avatar}
              alt="user's avatar"
            />
          </div>
          <div className={styles.text_info}>
            <div className={styles.name_and_role}>
              <p className={styles.username}>{username}</p>
              <p className={styles.user_role}>{user_role}</p>
            </div>
            <div className={styles.rating_container}>
              <FaStar className={styles.star_icon} />
              <p className={styles.rating}>
                5<span>(23)</span>
              </p>
            </div>
          </div>
        </div>
        <hr className={styles.divider} />
        <div className={styles.statistics}>
          <div className={styles.statistics_unit}>
            <p className={styles.statistics_text}>Ответы на новые сообщения</p>
            <div className={styles.statistics_progress}>
              <span
                className={`${styles.statistics_progress_circle} ${styles.circle_green}`}
              ></span>
              <p className={styles.statistics_result}>100%</p>
            </div>
          </div>
          <div className={styles.statistics_unit}>
            <p className={styles.statistics_text}>Время ответа на сообщения</p>
            <p
              className={`${styles.statistics_result} ${styles.not_percentage}`}
            >
              1 <span>час</span>
            </p>
          </div>
          <div className={styles.statistics_unit}>
            <p className={styles.statistics_text}>Заказы, отданные в срок</p>
            <div className={styles.statistics_progress}>
              <span
                className={`${styles.statistics_progress_circle} ${styles.circle_orange}`}
              ></span>
              <p className={styles.statistics_result}>75%</p>
            </div>
          </div>
          <div className={styles.statistics_unit}>
            <p className={styles.statistics_text}>Выполненные заказы</p>
            <div className={styles.statistics_progress}>
              <span
                className={`${styles.statistics_progress_circle} ${styles.circle_green}`}
              ></span>
              <p className={styles.statistics_result}>100%</p>
            </div>
          </div>
        </div>
      </div>
      <hr className={styles.divider} />
      <div className={styles.earnings}>
        <p className={styles.earnings_text}>Заработанно в {current_month}</p>
        <p className={styles.earnings_amount}>35 000 ₽</p>
      </div>
    </div>
  );
};

export default MetainfoWidget;
