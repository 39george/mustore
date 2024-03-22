import styles from "./AccountProducts.module.scss";
import {
  ProductSectionType,
  IProduct,
  TypeDeclension,
} from "../../types/types";
import { FC, useEffect } from "react";
import ProductSection from "../Components/UI_AccountProducts/ProductSection";
import { mock_songs } from "./mock_products";
import ProductDefault from "../Components/AccountProducts/ProductDefault";
import { useDispatch } from "react-redux";
import { set_product_status } from "../../state/product_status_slice";

const products: IProduct[][] = [];
products.push(mock_songs);

const AccountProducts: FC = () => {
  const dispatch = useDispatch();

  useEffect(() => {
    if (products.length !== 0) {
      dispatch(set_product_status("active"));
    } else {
      dispatch(set_product_status(null));
    }
  }, [products]);
  return (
    <div
      className={`${styles.products} ${
        products.length === 0 && styles.products_default_layout
      }`}
    >
      {products.length === 0 ? (
        <ProductDefault />
      ) : (
        <>
          <ProductSection
            type={ProductSectionType.songs}
            type_declension={TypeDeclension.song}
            new_declension="новую"
            products={products[0]}
            link="upload_song"
          />
          <hr className={styles.divider} />
        </>
      )}
    </div>
  );
};

export default AccountProducts;
