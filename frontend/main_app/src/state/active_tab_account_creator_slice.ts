import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { ActiveTabsAccountCreator } from "../types/types";

interface ActiveTabsAccountCreatorState {
  active_tab: string;
}
const regex = /\/personal-account\/([^\/]+)/;
let current_location = window.location.pathname;
let match = current_location.match(regex);

const initialState: ActiveTabsAccountCreatorState = {
  active_tab: match ? match[1] : "",
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
