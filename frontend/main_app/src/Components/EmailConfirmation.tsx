import styles from "./EmailConfirmation.module.scss";
import { NavLink, useNavigate } from "react-router-dom";
import logo from "../assets/svg/logo.svg";
import { FC } from "react";
import { useDispatch, useSelector } from "react-redux";
import { set_active_section } from "../state/active_section_slice";
import { RootState } from "../state/store";

const EmailConfirmation: FC = () => {
  const dispatch = useDispatch();
  const navigate = useNavigate();

  const handle_help_link_click = () => {
    dispatch(set_active_section(null));
  };

  const previous_path = useSelector<RootState, string>(
    (state) => state.previous_path.previous_path
  );

  const handle_return_link_click = () => {
    navigate(previous_path);
  };

  return (
    <div className={styles.email_confirmation}>
      <div className={styles.confirmation_container}>
        <div className={styles.content}>
          <h2 className={styles.h2}>Поздравляем!</h2>
          <p className={styles.p}>
            Вы успешно создали аккаунт на{" "}
            <span className={styles.harmonysphere}>
              HARMONY<span>.</span>SPHERE
            </span>
            <br />
            <br />
            На ваш email было отправлено письмо для подтверждения аккаунта.
            Пожалуйста, перейдите по ссылке из этого письма, чтобы завершить
            регистрацию.
          </p>
          <hr className={styles.divider} />
          <div className={styles.aux_info}>
            <NavLink
              to="../help"
              className={styles.help_link}
              onClick={handle_help_link_click}
            >
              Не приходит письмо?
            </NavLink>
            <img
              src={logo}
              alt="logo"
              className={styles.logo}
            />
            <p
              className={styles.return_link}
              onClick={handle_return_link_click}
            >
              Вернуться на сайт
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default EmailConfirmation;
