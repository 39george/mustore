import { useCallback, useState } from "react";
import { CheckedItems } from "../types/types";

const useCheckboxState = (init_state: CheckedItems = {}) => {
  const [checked_items, set_checked_items] = useState<CheckedItems>(init_state);

  const handle_checkbox_change = useCallback((item: string) => {
    set_checked_items((prev_checked_items) => {
      return { ...prev_checked_items, [item]: !prev_checked_items[item] };
    });
  }, []);

  return { checked_items, handle_checkbox_change };
};

export default useCheckboxState;
