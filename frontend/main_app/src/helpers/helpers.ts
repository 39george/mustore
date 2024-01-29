import { AxiosError } from "axios";

export const wait = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

export const handle_axios_error = (
  error: AxiosError,
  set_error: (message: string) => void
) => {
  if (error.response) {
    switch (error.response.status) {
      case 400:
        console.error("Bad request.", error.message);
        break;
      case 500:
        set_error(
          "Что-то не так с нашим сервером, мы уже работаем над этим. Пожалуйста, попробуйте обновить страницу"
        );
        console.error("Server responded with error: ", error.message);
        break;
      default:
        console.error("An unexpected error occured: ", error.message);
        break;
    }
  }
};
