import axios from "axios";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../../config";
import { wait } from "../../helpers/helpers";
import { useState } from "react";

const useLogInUserApi = () => {
  const [login_status, set_login_status] = useState<number | null>(null);
  const [login_error, set_login_error] = useState<string | null>(null);

  const post_data = async (data: string, attempts: number = 1) => {
    try {
      const response = await axios.post(`${API_URL}/login`, data, {
        headers: {
          "Content-Type": "application/json",
        },
      });
      set_login_status(response.status);
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 400:
              set_login_status(error.response.status);
              break;
            case 401:
              set_login_status(error.response.status);
              break;
            case 500:
              if (attempts < MAX_RETRIES) {
                await wait(RETRY_DELAY_MS);
                post_data(data, attempts + 1);
              } else {
                set_login_error(
                  "Что-то не так с нашим сервером, мы уже работаем над этим. Пожалуйста, попробуйте обновить страницу"
                );
              }
              break;
            default:
              console.error(
                "API error: ",
                error.response.status,
                error.response.data
              );
              set_login_error(
                "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
              );
              break;
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            post_data(data, attempts + 1);
          } else {
            set_login_error(
              "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
            );
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
        }
      } else {
        console.error("Non-Axios:", error);
      }
    }
  };

  return { login_status, login_error, post_data };
};

export default useLogInUserApi;
