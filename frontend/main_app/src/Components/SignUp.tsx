import styles from "./SignUp.module.scss";
import { FC, FormEvent, useEffect, useRef, useState } from "react";
import { HiMiniXMark } from "react-icons/hi2";
import { useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { z } from "zod";
import zxcvbn from "zxcvbn";
import { API_URL } from "../config";
import { RootState } from "../state/store";
import { FaEye } from "react-icons/fa";
import { TbEyeClosed } from "react-icons/tb";
import { FaTriangleExclamation } from "react-icons/fa6";
import { GoCheckCircleFill } from "react-icons/go";
import axios from "axios";

interface FromData {
  username: string;
  email: string;
  password: string;
  confirm_password: string;
}

type InputName = "password" | "confirm_password";

type InputNames = {
  [key in InputName]: boolean;
};

type InputType = "text" | "password";

type InputTypes = {
  [key in InputName]: InputType;
};

type UsernameStatus =
  | ""
  | "имя должно иметь не менее 3 символов"
  | "имя не должно иметь более 256 символов"
  | "это имя уже занято"
  | "имя содержит запрещенный символ"
  | "имя свободно";

type UsernameCheckProgress = "" | "unacceptable" | "pending" | "approved";

interface UsernameExistence {
  exists: boolean;
}

interface EmailInputInfo {
  success?: boolean;
  message?: string;
}

type PasswordStatus =
  | ""
  | "пароль не должен содержать пробелов"
  | "пароль должен быть короче 32 символов"
  | "очень слабый"
  | "слабый"
  | "предсказуемый"
  | "ненадежный"
  | "надежный";

interface ConfirmPasswordInfo {
  success?: boolean;
  message?: string;
}

const colors = {
  warning: "#EF0606",
  neutral: "#d9d9d9",
  neutral_outline: "#868381",
  password_too_weak: "#700000",
  password_middle: "#fe8c49",
  password_unreliable: "#E6A600",
  success: "#599c00",
};

const forbidden_symbols = /^[^\/\(\)"<>\{\}^\\;:\s*]*$/;

const SignUp: FC = () => {
  const navigate = useNavigate();
  const previous_path = useSelector<RootState, string>(
    (state) => state.previous_path.previous_path
  );
  const [form_data, set_form_data] = useState<FromData>({
    username: "",
    email: "",
    password: "",
    confirm_password: "",
  });
  const [input_type, set_input_type] = useState<InputTypes>({
    password: "password",
    confirm_password: "password",
  });
  const [username_status, set_username_status] = useState<UsernameStatus>("");
  const [username_check_porgress, set_username_check_progress] =
    useState<UsernameCheckProgress>("");
  const ref_username_check_progress = useRef<UsernameCheckProgress>("");
  const [username_border_color, set_username_border_color] =
    useState<string>("");
  const [email_input_info, set_email_input_info] = useState<EmailInputInfo>({});
  const email_schema = z.string().email();
  const [is_password_visible, set_is_password_visible] = useState<InputNames>({
    password: false,
    confirm_password: false,
  });
  const [password_status, set_password_status] = useState<PasswordStatus>("");
  const [password_status_bar_colors, set_password_status_bar_colors] = useState(
    {
      bar_1: colors.neutral,
      bar_2: colors.neutral,
      bar_3: colors.neutral,
      bar_4: colors.neutral,
      bar_5: colors.neutral,
    }
  );
  const [password_status_color, set_password_status_color] = useState("");
  const [input_disabled, set_input_disabled] = useState(true);
  const [confirm_password_info, set_confirm_password_info] =
    useState<ConfirmPasswordInfo>({});
  const [input_validity, set_input_validity] = useState({
    username: false,
    email: false,
    password: false,
    account_type: false,
  });

  // Handle input change and submit
  const handle_change = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    set_form_data((prev_state) => ({
      ...prev_state,
      [name]: value,
    }));
  };

  const handle_submit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    console.log(form_data);
  };

  // Checking username validity
  useEffect(() => {
    let timer: NodeJS.Timeout | undefined;

    if (form_data.username !== "") {
      if (timer) {
        clearTimeout(timer);
      }
      const is_data_correct = check_username_correctness(form_data.username);

      if (is_data_correct) {
        set_username_check_progress("pending");
        ref_username_check_progress.current = "pending";
        timer = setTimeout(() => {
          const check_existence = async () => {
            const username_exists = await check_is_username_exists(
              form_data.username
            );

            if (ref_username_check_progress.current !== "pending") {
              return;
            }

            if (username_exists) {
              set_username_status("это имя уже занято");
              set_username_check_progress("unacceptable");
              ref_username_check_progress.current = "unacceptable";
            } else {
              set_username_status("имя свободно");
              set_username_check_progress("approved");
              ref_username_check_progress.current = "approved";
            }
          };

          check_existence();
        }, 500);
      } else {
        set_username_check_progress("unacceptable");
        ref_username_check_progress.current = "unacceptable";
      }
    } else {
      if (timer) {
        clearTimeout(timer);
      }
      set_username_status("");
      set_username_check_progress("");
    }

    return () => {
      if (timer) {
        clearTimeout(timer);
      }
    };
  }, [form_data.username]);

  const check_username_correctness = (username: string) => {
    if (!forbidden_symbols.test(username)) {
      set_username_status("имя содержит запрещенный символ");
      return false;
    }
    if (username.length < 3) {
      set_username_status("имя должно иметь не менее 3 символов");
      return false;
    }
    if (username.length > 256) {
      set_username_status("имя не должно иметь более 256 символов");
      return false;
    }
    if (!forbidden_symbols.test(username)) {
      set_username_status("имя содержит запрещенный символ");
      return false;
    }
    return true;
  };

  const check_is_username_exists = async (username: string) => {
    try {
      const response = await axios.get<UsernameExistence>(
        `${API_URL}/username_status?username=${username}`
      );
      return response.data.exists;
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    switch (username_check_porgress) {
      case "":
        set_username_border_color("");
        break;
      case "unacceptable":
        set_username_border_color(`1px solid ${colors.warning}`);
        break;
      case "pending":
        set_username_border_color("");
        break;
      case "approved":
        set_username_border_color(`1px solid ${colors.success}`);
        break;
    }
  }, [username_check_porgress]);

  // Checking email validity
  useEffect(() => {
    if (form_data.email !== "") {
      const validation_info = email_schema.safeParse(form_data.email);
      if (validation_info.success) {
        set_email_input_info({
          success: true,
          message: "корректный email",
        });
        set_input_validity((prev) => ({
          ...prev,
          email: true,
        }));
      } else {
        set_email_input_info({
          success: false,
          message: "некорректный email",
        });
        set_input_validity((prev) => ({
          ...prev,
          email: false,
        }));
      }
    }
  }, [form_data.email]);

  // Checking password strenght
  useEffect(() => {
    if (form_data.password !== "") {
      if (form_data.password.includes(" ")) {
        set_password_status("пароль не должен содержать пробелов");
        set_password_status_color(colors.warning);
      } else if (form_data.password.length > 32) {
        set_password_status("пароль должен быть короче 32 символов");
        set_password_status_color(colors.warning);
      } else {
        let user_inputs: string[] = [];
        user_inputs.push(form_data.username);
        user_inputs.push(form_data.email);

        const password_strenght_info = zxcvbn(form_data.password, user_inputs);
        switch (password_strenght_info.score) {
          case 0:
            set_password_status("очень слабый");
            set_password_status_color(colors.password_too_weak);
            set_password_status_bar_colors({
              bar_1: colors.password_too_weak,
              bar_2: colors.neutral,
              bar_3: colors.neutral,
              bar_4: colors.neutral,
              bar_5: colors.neutral,
            });
            break;
          case 1:
            set_password_status("слабый");
            set_password_status_color(colors.warning);
            set_password_status_bar_colors({
              bar_1: colors.warning,
              bar_2: colors.warning,
              bar_3: colors.neutral,
              bar_4: colors.neutral,
              bar_5: colors.neutral,
            });
            break;
          case 2:
            set_password_status("предсказуемый");
            set_password_status_color(colors.password_middle);
            set_password_status_bar_colors({
              bar_1: colors.password_middle,
              bar_2: colors.password_middle,
              bar_3: colors.password_middle,
              bar_4: colors.neutral,
              bar_5: colors.neutral,
            });
            break;
          case 3:
            set_password_status("ненадежный");
            set_password_status_color(colors.password_unreliable);
            set_password_status_bar_colors({
              bar_1: colors.password_unreliable,
              bar_2: colors.password_unreliable,
              bar_3: colors.password_unreliable,
              bar_4: colors.password_unreliable,
              bar_5: colors.neutral,
            });
            break;
          case 4:
            set_password_status("надежный");
            set_password_status_color(colors.success);
            set_password_status_bar_colors({
              bar_1: colors.success,
              bar_2: colors.success,
              bar_3: colors.success,
              bar_4: colors.success,
              bar_5: colors.success,
            });
            set_input_disabled(false);
            break;
        }

        if (password_strenght_info.score !== 4) {
          set_input_disabled(true);
        }
      }
    } else {
      set_password_status("");
      set_input_disabled(true);
    }
  }, [form_data.password]);

  // Checking password match
  useEffect(() => {
    if (form_data.confirm_password !== "") {
      if (form_data.password === form_data.confirm_password) {
        set_confirm_password_info({
          success: true,
          message: "пароли совпадают",
        });
      } else {
        set_confirm_password_info({
          success: false,
          message: "пароли должны совпадать",
        });
      }
    }
  }, [form_data.confirm_password, form_data.password]);

  // Handling click on the eye icon
  const on_eye_click = (input_name: InputName) => {
    if (input_name === "confirm_password") {
      if (input_disabled) {
        return;
      }
    }
    set_is_password_visible((prev) => ({
      ...prev,
      [input_name]: !prev[input_name],
    }));
  };

  useEffect(() => {
    if (is_password_visible.password) {
      set_input_type((prev) => ({
        ...prev,
        password: "text",
      }));
    } else {
      set_input_type((prev) => ({
        ...prev,
        password: "password",
      }));
    }

    if (is_password_visible.confirm_password) {
      set_input_type((prev) => ({
        ...prev,
        confirm_password: "text",
      }));
    } else {
      set_input_type((prev) => ({
        ...prev,
        confirm_password: "password",
      }));
    }
  }, [is_password_visible.password, is_password_visible.confirm_password]);

  // Handling returning to the previous page
  const handle_close = () => {
    navigate(previous_path);
  };

  // Rendering component
  return (
    <div className={styles.sign_up_window}>
      <HiMiniXMark
        className={styles.close_icon}
        onClick={handle_close}
      />
      <div className={styles.log_in_section}>
        <div className={styles.log_in_content}>
          <p className={styles.p_log_in}>Уже есть аккаунт?</p>
          <button className={styles.button_log_in}>Войти</button>
        </div>
      </div>
      <div className={styles.sign_up_section}>
        <div className={styles.sign_up_content}>
          <p className={styles.p_join_us}>присоединяйтесь к сообществу</p>
          <h1 className={styles.title}>
            HARMONY<span>.</span>
            {/* <br /> */}
            SPHERE
          </h1>
          <h2 className={styles.tagline}>
            Сервис для тех, кто хочет найти или предложить свои{" "}
            <span>музыкальные решения.</span>
          </h2>
          <form
            onSubmit={handle_submit}
            className={styles.sign_up_form}
          >
            <div className={styles.input_block}>
              <div className={styles.input_container}>
                <input
                  type="text"
                  name="username"
                  placeholder="Имя пользователя"
                  onChange={handle_change}
                  className={styles.sign_up_input}
                  style={{
                    border: `${username_border_color}`,
                  }}
                  required
                />
              </div>
              {username_check_porgress === "pending" && (
                <div className={styles.loader}></div>
              )}
              {form_data.username !== "" &&
                username_check_porgress !== "pending" && (
                  <div className={styles.input_info}>
                    <div>
                      {username_check_porgress === "approved" ? (
                        <GoCheckCircleFill className={styles.success_icon} />
                      ) : (
                        <FaTriangleExclamation
                          className={styles.warning_icon}
                        />
                      )}
                    </div>
                    <p
                      className={`${styles.info_message} ${
                        username_check_porgress === "approved"
                          ? styles.info_success
                          : styles.info_warning
                      }`}
                    >
                      {username_status}
                    </p>
                  </div>
                )}
            </div>
            <div className={styles.input_block}>
              <div className={styles.input_container}>
                <input
                  type="text"
                  name="email"
                  onChange={handle_change}
                  placeholder="Email"
                  className={styles.sign_up_input}
                  style={{
                    border: `${
                      form_data.email !== ""
                        ? email_input_info.success
                          ? `1px solid ${colors.success}`
                          : `1px solid ${colors.warning}`
                        : ""
                    }`,
                  }}
                  required
                />
              </div>
              {form_data.email !== "" && (
                <div className={styles.input_info}>
                  <div>
                    {email_input_info.success ? (
                      <GoCheckCircleFill className={styles.success_icon} />
                    ) : (
                      <FaTriangleExclamation className={styles.warning_icon} />
                    )}
                  </div>
                  <p
                    className={`${styles.info_message} ${
                      email_input_info.success
                        ? styles.info_success
                        : styles.info_warning
                    }`}
                  >
                    {email_input_info.message}
                  </p>
                </div>
              )}
            </div>
            <div className={styles.input_block}>
              <div className={styles.input_container}>
                <input
                  type={input_type.password}
                  name="password"
                  onChange={handle_change}
                  placeholder="Пароль"
                  className={styles.sign_up_input}
                  style={{
                    border: `${
                      password_status !== ""
                        ? `1px solid ${password_status_color}`
                        : ""
                    }`,
                  }}
                  required
                />
                <div
                  className={styles.eye_icon}
                  onClick={() => on_eye_click("password")}
                >
                  {is_password_visible.password ? <FaEye /> : <TbEyeClosed />}
                </div>
              </div>
              {password_status !== "" && (
                <>
                  <div className={styles.input_info}>
                    <div>
                      {password_status === "надежный" ? (
                        <GoCheckCircleFill
                          className={styles.success_icon}
                          style={{ color: `${password_status_color}` }}
                        />
                      ) : (
                        <FaTriangleExclamation
                          className={styles.warning_icon}
                          style={{ color: `${password_status_color}` }}
                        />
                      )}
                    </div>
                    <p
                      className={styles.info_message}
                      style={{ color: `${password_status_color}` }}
                    >
                      {password_status}
                    </p>
                  </div>
                  {password_status !== "пароль не должен содержать пробелов" &&
                    password_status !==
                      "пароль должен быть короче 32 символов" && (
                      <div className={styles.password_status_bar}>
                        <div
                          className={styles.bar}
                          style={{
                            backgroundColor: `${password_status_bar_colors.bar_1}`,
                          }}
                        ></div>
                        <div
                          className={styles.bar}
                          style={{
                            backgroundColor: `${password_status_bar_colors.bar_2}`,
                          }}
                        ></div>
                        <div
                          className={styles.bar}
                          style={{
                            backgroundColor: `${password_status_bar_colors.bar_3}`,
                          }}
                        ></div>
                        <div
                          className={styles.bar}
                          style={{
                            backgroundColor: `${password_status_bar_colors.bar_4}`,
                          }}
                        ></div>
                        <div
                          className={styles.bar}
                          style={{
                            backgroundColor: `${password_status_bar_colors.bar_5}`,
                          }}
                        ></div>
                      </div>
                    )}
                </>
              )}
            </div>
            <div
              className={styles.input_block}
              style={{ opacity: `${input_disabled ? "0.5" : "1"}` }}
            >
              <div className={styles.input_container}>
                <input
                  type={input_type.confirm_password}
                  name="confirm_password"
                  onChange={handle_change}
                  placeholder="Подтвердите пароль"
                  className={styles.sign_up_input}
                  style={{
                    border: `${
                      form_data.confirm_password !== ""
                        ? confirm_password_info.success
                          ? `1px solid ${colors.success}`
                          : `1px solid ${colors.warning}`
                        : ""
                    }`,
                  }}
                  disabled={input_disabled}
                  required
                />
                <div
                  className={styles.eye_icon}
                  style={{
                    cursor: `${input_disabled ? "default" : "pointer"}`,
                  }}
                  onClick={() => on_eye_click("confirm_password")}
                >
                  {is_password_visible.confirm_password ? (
                    <FaEye />
                  ) : (
                    <TbEyeClosed />
                  )}
                </div>
              </div>
              {form_data.confirm_password !== "" && (
                <div className={styles.input_info}>
                  <div>
                    {confirm_password_info.success ? (
                      <GoCheckCircleFill className={styles.success_icon} />
                    ) : (
                      <FaTriangleExclamation className={styles.warning_icon} />
                    )}
                  </div>
                  <p
                    className={`${styles.info_message} ${
                      confirm_password_info.success
                        ? styles.info_success
                        : styles.info_warning
                    }`}
                  >
                    {confirm_password_info.message}
                  </p>
                </div>
              )}
            </div>
            {/* <div className={styles.input_block}>
              <div
                className={`${styles.input_container} ${styles.account_types}`}
              >
                <div className={styles.account_type_header}>
                  <p className={styles.account_type_p}>Выберите тип аккаунта</p>
                  <GoChevronDown className={styles.chevron_icon} />
                </div>
                <div className={styles.account_types_container}>
                  <p className={styles.account_type}>Автор</p>
                  <p className={styles.account_type}>Покупатель</p>
                </div>
              </div>
            </div> */}
            <button type="submit">Присоединиться</button>
          </form>
        </div>
      </div>
    </div>
  );
};

export default SignUp;
