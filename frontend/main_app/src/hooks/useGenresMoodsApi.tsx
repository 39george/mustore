import { useCallback, useEffect, useState } from "react";
import { API_URL } from "../config";
import axios, { AxiosError } from "axios";

const MAX_RETRIES = 3;
const RETRY_DELAY_MS = 400;

type GenreOrMood = string[];

const useGenresMoodsApi = (endpoint: string) => {
  const [data, set_data] = useState<GenreOrMood>([]);
  const [error, set_error] = useState<string | null>(null);

  const wait = (ms: number) => {
    return new Promise((resolve) => setTimeout(resolve, ms));
  };

  const fetch_data = useCallback(
    async (attempts: number = 1) => {
      try {
        const response = await axios.get<GenreOrMood>(
          `${API_URL}/open/${endpoint}`
        );
        if (Array.isArray(response.data)) {
          set_data(response.data);
        } else {
          console.error("Unexpected response type:", response.data);
        }
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
              fetch_data(attempts + 1);
            } else {
              handle_axios_error(error);
            }
          } else if (error.request) {
            if (attempts < MAX_RETRIES) {
              await wait(RETRY_DELAY_MS);
              fetch_data(attempts + 1);
            } else {
              set_error(
                "No response from server. Please, check your internet connection and try again"
              );
            }
          } else {
            set_error("Error in setting up the request.");
            console.error("API Error: Reqest setup error:", error.message);
          }
        } else {
          set_error("An unexpected error occured.");
          console.error("Non-Axios:", error);
        }
      }
    },
    [endpoint]
  );

  const handle_axios_error = (error: AxiosError) => {
    if (error.response) {
      switch (error.response.status) {
        case 400:
          set_error("Bad input, please check your request and try again");
          break;
        case 500:
          set_error("Internal server error. Please try again later.");
          break;
        default:
          set_error("An unexpected error occured. Please, try again later.");
          break;
      }
    }
  };

  useEffect(() => {
    let is_mounted = true;

    if (is_mounted) {
      fetch_data();
    }

    return () => {
      is_mounted = false;
    };
  }, [fetch_data]);

  return { data, error, retry: fetch_data };
};

export default useGenresMoodsApi;
