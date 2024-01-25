import axios from "axios";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../config";
import { handle_axios_error, wait } from "../helpers/helpers";
import { useState } from "react";

const useSignUpUserApi = () => {
  const [error_data, set_error_data] = useState<string | null>();

  const post_data = async (
    data: string,
    attempts: number = 1
  ): Promise<number | null | undefined> => {
    try {
      const response = await axios.post(`${API_URL}/signup`, data, {
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
      });

      return response.status;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          console.error(
            "API Error:",
            error.response.status,
            error.response.data
          );

          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            return post_data(data, attempts + 1);
          } else {
            handle_axios_error(error, set_error_data);
            return null;
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            return post_data(data, attempts + 1);
          } else {
            set_error_data(
              "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
            );
            return null;
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
        }
      } else {
        console.error("Non-Axios:", error);
      }
    }
  };

  return { error_data, post_data };
};

export default useSignUpUserApi;
