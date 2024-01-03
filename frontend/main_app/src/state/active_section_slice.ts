import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export type ActiveSection =
  | "hero"
  | "why_us"
  | "group"
  | "authors_reviews"
  | null;

interface ActiveSectionState {
  active_section: ActiveSection;
}

const initialState: ActiveSectionState = {
  active_section: null,
};

export const active_section_slice = createSlice({
  name: "active_section",
  initialState,
  reducers: {
    set_active_section: (
      state: ActiveSectionState,
      action: PayloadAction<ActiveSection>
    ) => {
      state.active_section = action.payload;
    },
  },
});

export const { set_active_section } = active_section_slice.actions;

export const select_active_section = (state: ActiveSectionState) =>
  state.active_section;

export default active_section_slice.reducer;
