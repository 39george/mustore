import styles from "./AccountProducts.module.scss";
import {
  ProductSectionType,
  IProduct,
  TypeDeclension,
} from "../../types/types";
import { FC } from "react";
import ProductSection from "../Components/UI_AccountProducts/ProductSection";
import ProductDefault from "../Components/UI_AccountProducts/ProductDefault";
import { mock_songs } from "./mock_products";

const products: IProduct[][] = [];
// products.push(mock_songs);

const AccountProducts: FC = () => {
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
          />
          <hr className={styles.divider} />
        </>
      )}
    </div>
  );
};

export default AccountProducts;
