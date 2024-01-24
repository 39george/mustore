import { useState } from "react";
import { API_URL } from "../config";
import axios from "axios";
import { UsernameExistence } from "../types/types";
import { handle_axios_error, wait } from "../helpers/helpers";

const MAX_RETRIES = 3;
const RETRY_DELAY_MS = 1000;

const useCheckUsernameExistnece = () => {
  const [error, set_error] = useState<string | null>();

  const fetch_data = async (username: string, attempts: number = 1) => {
    try {
      const response = await axios.get<UsernameExistence>(
        `${API_URL}/username_status?username=${username}`
      );
      return response.data;
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
            fetch_data(username, attempts + 1);
          } else {
            handle_axios_error(error, set_error);
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            fetch_data(username, attempts + 1);
          } else {
            set_error(
              "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
            );
          }
        } else {
          console.error("API Error: Reqest setup error:", error.message);
        }
      } else {
        console.error("Non-Axios:", error);
      }
      return null;
    }
  };

  return { error, check_is_username_exists: fetch_data };
};

export default useCheckUsernameExistnece;
