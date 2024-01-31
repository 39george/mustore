import { AxiosError } from "axios";

// Error handling
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

// Handling moving focus to the next input by pressing `enter` key
type InputRef = React.RefObject<HTMLInputElement>;

export const handle_enter_key_down = (
  e: React.KeyboardEvent<HTMLInputElement>,
  idx: number,
  input_refs: InputRef[]
) => {
  if (e.key === "Enter") {
    if (input_refs[idx].current?.value) {
      e.preventDefault();
      if (idx < input_refs.length - 1) {
        input_refs[idx + 1].current?.focus();
      }
    }
  }
};
