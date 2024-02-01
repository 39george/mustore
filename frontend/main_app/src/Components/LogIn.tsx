import styles from "./LogIn.module.scss";
import { FC, FormEvent, useEffect, useRef, useState } from "react";
import { HiMiniXMark } from "react-icons/hi2";
import { FaTriangleExclamation } from "react-icons/fa6";
import { handle_enter_key_down } from "../helpers/helpers";
import { NavLink, useNavigate } from "react-router-dom";
import { useSelector } from "react-redux";
import { RootState } from "../state/store";
import useLogInUserApi from "../hooks/API/useLogInUserApi";
import ErrorWindow from "./ErrorWindow";
import useCheckPermissionsApi from "../hooks/API/useCheckPermissionsApi";

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
  const [button_disabled, set_button_disabled] = useState(true);
  const [button_class_name, set_button_class_name] = useState("");
  const [login_in_progress, set_login_in_porgress] = useState(false);
  const { login_status, login_error, post_data } = useLogInUserApi();
  const { check_user_permissions } = useCheckPermissionsApi();

  // Handling submit and input change
  const handle_submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    set_login_in_porgress(true);

    const try_to_login = async () => {
      await post_data(JSON.stringify(form_data));
    };

    try_to_login();
  };

  useEffect(() => {
    if (login_status !== null) {
      if (login_status === 200) {
        setTimeout(() => {
          set_login_in_porgress(false);
          check_user_permissions();
          handle_close();
        }, 500);
      } else {
        setTimeout(() => {
          set_login_in_porgress(false);
          set_wrong_data(true);
        }, 500);
      }
    }

    if (login_error) {
      set_login_in_porgress(false);
    }
  }, [login_status, login_error]);

  const handle_input_change = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    set_form_data((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  // Handling enable/disable submit button and hover
  useEffect(() => {
    if (form_data.email && form_data.password !== "") {
      set_button_disabled(false);
    } else {
      set_button_disabled(true);
    }
  }, [form_data]);

  const handle_mouse_enter = () => {
    set_button_class_name(`${styles.button_hover}`);
  };
  const handle_mouse_leave = () => {
    set_button_class_name("");
  };

  // Handling returning to the previous page
  const handle_close = () => {
    navigate(previous_path);
  };

  return (
    <div className={styles.login_window}>
      {login_error && <ErrorWindow error_message={login_error} />}
      {login_in_progress && (
        <div className={styles.loader_bg}>
          <div className={styles.loader_big}></div>
        </div>
      )}
      <div className={styles.upper_bar}>
        <div className={styles.sign_up}>
          <p className={styles.sign_up_p}>Нет аккаунта?</p>
          <NavLink
            to="../signup"
            className={styles.sign_up_button}
          >
            Создать
          </NavLink>
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
              style={{ border: `${wrong_data ? "1px solid #ef0606" : ""}` }}
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
              style={{ border: `${wrong_data ? "1px solid #ef0606" : ""}` }}
              onChange={handle_input_change}
              onKeyDown={(e) => handle_enter_key_down(e, 1, input_refs)}
            />
            <p className={styles.forgot_password_prompt}>Забыли пароль?</p>
          </div>
          <button
            type="submit"
            disabled={button_disabled}
            onMouseEnter={handle_mouse_enter}
            onMouseLeave={handle_mouse_leave}
            className={button_class_name}
            style={
              button_disabled
                ? { opacity: "0.5", cursor: "default" }
                : { opacity: "1", cursor: "pointer" }
            }
          >
            Войти
          </button>
        </form>
      </div>
      <div className={styles.waves_decor} />
    </div>
  );
};

export default LogIn;
