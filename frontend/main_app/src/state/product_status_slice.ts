import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { ProductStatus } from "../types/types";

interface InitState {
  product_status: ProductStatus;
}

const initialState: InitState = {
  product_status: null,
};

const product_status_slice = createSlice({
  name: "product_status_slice",
  initialState: initialState,
  reducers: {
    set_product_status: (
      state: InitState,
      action: PayloadAction<ProductStatus>
    ) => {
      state.product_status = action.payload;
    },
  },
});

export const { set_product_status } = product_status_slice.actions;

export default product_status_slice.reducer;
