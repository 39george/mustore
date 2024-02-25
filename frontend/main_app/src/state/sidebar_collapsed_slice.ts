import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface SidebarCollapsedState {
  sidebar_collapsed: boolean;
}

const initialState: SidebarCollapsedState = {
  sidebar_collapsed: window.innerWidth <= 950,
};

export const sidebar_collapsed_slice = createSlice({
  name: "sidebar_collapsed",
  initialState: initialState,
  reducers: {
    set_sidebar_collapsed: (
      state: SidebarCollapsedState,
      action: PayloadAction<boolean>
    ) => {
      state.sidebar_collapsed = action.payload;
    },
  },
});

export const { set_sidebar_collapsed } = sidebar_collapsed_slice.actions;

export default sidebar_collapsed_slice.reducer;
