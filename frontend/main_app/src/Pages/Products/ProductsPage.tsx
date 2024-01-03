import { FC } from "react";
import Filler from "../../UI/Filler";
import { Outlet } from "react-router-dom";

const ProductsPage: FC = () => {
  return (
    <>
      <Filler />
      <Outlet />
    </>
  );
};

export default ProductsPage;
