import styles from "./Services.module.scss";
import { FC } from "react";
import songwriting from "../../../assets/icons/songwriting.svg";
import mixing from "../../../assets/icons/mixing.svg";
import beatmaking from "../../../assets/icons/beatmaking.svg";
import ghostwriting from "../../../assets/icons/ghostwriting.svg";
import cover_design from "../../../assets/icons/cover_design.svg";
import { IServiceItem } from "../../../types/types";
import ServiceItem from "./UI/ServiceItem";

const services: IServiceItem[] = [
  {
    icon: songwriting,
    title: "Создание песни",
    description: `Дайте жизнь вашим мелодиям. Не важно, начинаете ли вы с пустого
              листа или уже имеете замысел, наши авторы помогут вам создать
              песню, которая тронет сердца. Любая мысль воплотится в
              произведение искусства.`,
  },
  {
    icon: mixing,
    title: "Сведение / Мастеринг",
    description: `Услуги сведения и мастеринга выведут ваш звук на новый уровень,
              обеспечив профессиональное звучание по мировым стандартам с нашими
              инженерами, придающими жизнь каждой ноте вашего творения.`,
  },
  {
    icon: beatmaking,
    title: "Написание бита",
    description: `Наши саундпродюсеры готовы предложить уникальные музыкальные
              ритмы, которые подогреют вашу творческую страсть. Мы создадим
              идеальную музыкальную основу для вашего будущего хита, независимо
              от жанра.`,
  },
  {
    icon: ghostwriting,
    title: "Гострайтинг",
    description: `Мы пишем, вы выступаете. У вас есть идея, но нет слов? Наши
              гострайтеры могут написать текст песни по вашим указаниям,
              сохраняя ваш уникальный стиль.`,
  },
  {
    icon: cover_design,
    title: "Дизайн обложки",
    description: `Привлекательная обложка может рассказать историю вашей песни еще
              до первой ноты. Наши дизайнеры создадут захватывающую
              визуализацию, которая уловит дух вашей музыки и позволит
              слушателям влюбиться в трек с первого взгляда.`,
  },
];

const Services: FC = () => {
  return (
    <section className={styles.services_section}>
      <h2 className={styles.h2}>Или закажите необходимую услугу</h2>
      <div className={styles.services_container}>
        {services.map((service, idx) => {
          return (
            <ServiceItem
              key={idx}
              icon={service.icon}
              title={service.title}
              description={service.description}
            />
          );
        })}
      </div>
    </section>
  );
};

export default Services;
