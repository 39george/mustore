import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { ActiveTabsAccountCreator } from "../types/types";

interface ActiveTabsAccountCreatorState {
  active_tab: ActiveTabsAccountCreator;
}

const initialState: ActiveTabsAccountCreatorState = {
  active_tab: "dashboard",
};

const active_tab_account_creator_slice = createSlice({
  name: "active_tab_account_creator",
  initialState: initialState,
  reducers: {
    set_active_tab_account_creator: (
      state: ActiveTabsAccountCreatorState,
      action: PayloadAction<ActiveTabsAccountCreator>
    ) => {
      state.active_tab = action.payload;
    },
  },
});

export const { set_active_tab_account_creator } =
  active_tab_account_creator_slice.actions;

export default active_tab_account_creator_slice.reducer;
