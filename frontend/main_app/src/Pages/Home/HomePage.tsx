import AuthorsReviews from "./Components/AuthorsReviews";
import Hero from "./Components/Hero";
import JoinUs from "./Components/JoinUs";
import Products from "./Components/Products";
import Services from "./Components/Services";
import WhyUs from "./Components/WhyUs";

const HomePage = () => {
  return (
    <>
      <Hero />
      <WhyUs />
      <Products />
      <Services />
      <JoinUs />
      <AuthorsReviews />
    </>
  );
};

export default HomePage;
