import styles from "./LogIn.module.scss";
import { FC, FormEvent, useRef, useState } from "react";
import { HiMiniXMark } from "react-icons/hi2";
import { FaTriangleExclamation } from "react-icons/fa6";
import { handle_enter_key_down } from "../helpers/helpers";
import { NavLink, useNavigate } from "react-router-dom";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import axios from "axios";
import { API_URL } from "../config";

interface FormData {
  email: string;
  password: string;
}

const LogIn: FC = () => {
  const navigate = useNavigate();
  const previous_path = useSelector<RootState, string>(
    (state) => state.previous_path.previous_path
  );
  const [form_data, set_form_data] = useState<FormData>({
    email: "",
    password: "",
  });
  const input_refs = [
    useRef<HTMLInputElement>(null),
    useRef<HTMLInputElement>(null),
  ];
  const [wrong_data, set_wrong_data] = useState(false);

  // Handling submit and input change
  const handle_submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    const try_to_login = async () => {
      try {
        const response = await axios.post(
          `${API_URL}/login`,
          JSON.stringify(form_data),
          {
            headers: {
              "Content-Type": "application/json",
            },
          }
        );
        console.log(response.status);
        set_wrong_data(false);
      } catch (error) {
        console.error(error);
        set_wrong_data(true);
      }
    };

    try_to_login();
  };

  const handle_input_change = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    set_form_data((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  // Handling returning to the previous page
  const handle_close = () => {
    navigate(previous_path);
  };

  return (
    <div className={styles.login_window}>
      <div className={styles.upper_bar}>
        <div className={styles.sign_up}>
          <p className={styles.sign_up_p}>Ещё нет аккаунта?</p>
          <button className={styles.sign_up_button}>
            <NavLink to="../signup">Создать</NavLink>
          </button>
        </div>
        <HiMiniXMark
          className={styles.close_icon}
          onClick={handle_close}
        />
      </div>
      <div className={styles.main_content}>
        <p className={styles.p_welcome}>Добро пожаловать!</p>
        <h1 className={styles.title}>
          HARMONY<span>.</span>
          SPHERE
        </h1>
        <p className={styles.p_login}>Вход в аккаунт</p>
        <form
          onSubmit={handle_submit}
          className={styles.form_login}
        >
          {wrong_data && (
            <div className={styles.input_info}>
              <FaTriangleExclamation className={styles.warning_icon} />
              <p className={styles.info_message}>неверный email или пароль</p>
            </div>
          )}
          <div className={styles.input_block}>
            <input
              type="text"
              name="email"
              placeholder="Email"
              ref={input_refs[0]}
              className={styles.login_input}
              onChange={handle_input_change}
              onKeyDown={(e) => handle_enter_key_down(e, 0, input_refs)}
              autoFocus
            />
          </div>
          <div className={styles.input_block}>
            <input
              type="password"
              name="password"
              placeholder="Пароль"
              ref={input_refs[1]}
              className={styles.login_input}
              onChange={handle_input_change}
              onKeyDown={(e) => handle_enter_key_down(e, 1, input_refs)}
            />
            <p className={styles.forgot_password_prompt}>Забыли пароль?</p>
          </div>
          <button type="submit">Войти</button>
        </form>
      </div>
      <div className={styles.waves_decor} />
    </div>
  );
};

export default LogIn;
