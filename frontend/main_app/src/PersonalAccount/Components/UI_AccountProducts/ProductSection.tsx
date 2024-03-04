import styles from "./ProductSection.module.scss";
import { FiPlus } from "react-icons/fi";
import { FC, useState } from "react";
import {
  IProduct,
  ProductSectionType,
  TypeDeclension,
} from "../../../types/types";
import ProductMeta from "./ProductMeta";
import ProductCoversContainer from "./ProductCoversContainer";

type NewDeclension = "новый" | "новую";

interface ProductSectionProps {
  type: ProductSectionType;
  type_declension: TypeDeclension;
  new_declension: NewDeclension;
  products: IProduct[];
}

const ProductSection: FC<ProductSectionProps> = ({
  type,
  type_declension,
  new_declension,
  products,
}) => {
  const [product_idx, set_product_idx] = useState(0);
  return (
    <div className={styles.product_section}>
      <div className={styles.product_header}>
        <h2 className={styles.h2}>{type}</h2>
        <div className={styles.upload_product}>
          <p className={styles.upload_product_p}>
            загрузить {new_declension} {type_declension}
          </p>
          <FiPlus className={styles.plus_icon} />
        </div>
      </div>
      <div className={styles.product_content}>
        <div className={styles.select_product}>
          <ProductCoversContainer products={products} />
          <ol className={styles.product_list}>
            {products.map((product, idx) => {
              return (
                <li
                  className={styles.product_list_item}
                  key={idx}
                >
                  <p className={styles.name}>{product.name}</p>
                  <p className={styles.duration}>{product.duration}</p>
                </li>
              );
            })}
          </ol>
        </div>
        <ProductMeta
          likes_count={products[product_idx].likes_count}
          listenings_count={products[product_idx].listenings_count}
          lyric={products[product_idx].lyric}
          moods={products[product_idx].moods}
          music_key={products[product_idx].music_key}
          name={products[product_idx].name}
          price={products[product_idx].price}
          primary_genre={products[product_idx].primary_genre}
          secondary_genre={products[product_idx].secondary_genre}
          sex={products[product_idx].sex}
          song_id={products[product_idx].song_id}
          tempo={products[product_idx].tempo}
        />
      </div>
    </div>
  );
};

export default ProductSection;
