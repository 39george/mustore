import { UserPermissions } from "../state/user_permissions_slice";
import { CheckedItems, UserRole } from "../types/types";

// Wait function
export const wait = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
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

// Find and translate user role
export const find_user_role_index = (
  arr: UserPermissions[],
  role: UserRole
) => {
  switch (role) {
    case "creator":
      return arr.findIndex((obj) => obj.name === "creator");
    case "consumer":
      return arr.findIndex((obj) => obj.name === "consumer");
  }
};
export const translate_user_role = (role: string) => {
  if (role === "creator") {
    return "Автор";
  } else {
    return "Покупатель";
  }
};

// Check if a checked object has any `true` value
export const no_true_values = (obj: CheckedItems) => {
  return Object.values(obj).every((value) => value === false);
};
