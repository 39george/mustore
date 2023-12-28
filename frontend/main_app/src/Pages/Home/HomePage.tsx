import { useRef } from "react";
import AuthorsReviews from "./Components/AuthorsReviews";
import Hero from "./Components/Hero";
import JoinUs from "./Components/JoinUs";
import Products from "./Components/Products";
import Services from "./Components/Services";
import WhyUs from "./Components/WhyUs";

const HomePage = () => {
  const why_us_ref = useRef<HTMLDivElement>(null);

  const scroll_to_why_us = () => {
    why_us_ref.current?.scrollIntoView({ behavior: "smooth" });
  };

  return (
    <>
      <Hero scroll_to_why_us={scroll_to_why_us} />
      <div ref={why_us_ref}>
        <WhyUs />
      </div>
      <Products />
      <Services />
      <JoinUs />
      <AuthorsReviews />
    </>
  );
};

export default HomePage;
