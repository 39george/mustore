import { PayloadAction, createSlice } from "@reduxjs/toolkit";

type SidebarTitle = "H.S" | "HARMONY.SPHERE";

type DisplayStyle = "none" | "block";

interface SidebarActionsState {
  sidebar_collapsed: boolean;
  sidebar_title: SidebarTitle;
  sidebar_chevron_display: DisplayStyle;
}

const initialState: SidebarActionsState = {
  sidebar_collapsed: window.innerWidth <= 950,
  sidebar_title: window.innerWidth <= 950 ? "H.S" : "HARMONY.SPHERE",
  sidebar_chevron_display: window.innerWidth <= 950 ? "none" : "block",
};

export const sidebar_actions_slice = createSlice({
  name: "sidebar_actions",
  initialState: initialState,
  reducers: {
    set_sidebar_collapsed: (
      state: SidebarActionsState,
      action: PayloadAction<boolean>
    ) => {
      state.sidebar_collapsed = action.payload;
    },
    set_sidebar_title: (
      state: SidebarActionsState,
      action: PayloadAction<SidebarTitle>
    ) => {
      state.sidebar_title = action.payload;
    },
    set_sidebar_chevron_display: (
      state: SidebarActionsState,
      action: PayloadAction<DisplayStyle>
    ) => {
      state.sidebar_chevron_display = action.payload;
    },
  },
});

export const {
  set_sidebar_collapsed,
  set_sidebar_title,
  set_sidebar_chevron_display,
} = sidebar_actions_slice.actions;

export default sidebar_actions_slice.reducer;
