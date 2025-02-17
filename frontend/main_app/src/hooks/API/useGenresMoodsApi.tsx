import { useState } from "react";
import { API_URL, MAX_RETRIES, RETRY_DELAY_MS } from "../../config";
import axios from "axios";
import { wait } from "../../helpers/helpers";

type GenreOrMood = string[];

const useGenresMoodsApi = () => {
  const [data, set_data] = useState<GenreOrMood>([]);
  const [error, set_error] = useState<string | null>(null);

  const fetch_data = async (
    endpoint: string,
    signal: AbortSignal,
    attempts: number = 1
  ) => {
    try {
      const response = await axios.get<GenreOrMood>(
        `${API_URL}/open/${endpoint}`,
        { signal: signal }
      );
      if (Array.isArray(response.data)) {
        const upper_cased = response.data.map((string) => {
          if (string.includes("&")) {
            return string.toUpperCase();
          }
          return string.charAt(0).toUpperCase() + string.slice(1);
        });
        set_data(upper_cased);
      } else {
        console.error("Unexpected response type:", response.data);
      }
    } catch (error) {
      if (axios.isAxiosError(error)) {
        if (error.response) {
          switch (error.response.status) {
            case 400:
              console.error("Bad request.", error.message);
              break;
            case 500:
              if (attempts < MAX_RETRIES) {
                await wait(RETRY_DELAY_MS);
                fetch_data(endpoint, signal, attempts + 1);
              } else {
                set_error(
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
              set_error(
                "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
              );
              break;
          }
        } else if (error.request) {
          if (attempts < MAX_RETRIES) {
            await wait(RETRY_DELAY_MS);
            fetch_data(endpoint, signal, attempts + 1);
          } else {
            set_error(
              "Нет ответа от сервера, пожалуйста, проверьте соединение с интернетом и попробуйте еще раз"
            );
          }
        } else {
          if (error.name !== "CanceledError") {
            console.error("API Error: Reqest setup error:", error.message);
          }
        }
      } else {
        console.error("Non-Axios:", error);
      }
    }
  };

  return { data, error, fetch_data };
};

export default useGenresMoodsApi;
