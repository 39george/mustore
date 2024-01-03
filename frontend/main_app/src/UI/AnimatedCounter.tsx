import styles from "./AnimatedCounter.module.scss";
import { FC, useEffect, useRef, useState } from "react";

interface AnimatedCounterProps {
  amount: number;
  duration: number;
  name: string;
}

const AnimatedCounter: FC<AnimatedCounterProps> = ({
  amount,
  duration,
  name,
}) => {
  const [count, set_count] = useState(0);
  const count_ref = useRef(count);
  const p_ref = useRef<HTMLParagraphElement>(null);

  // Increment the counter
  const increment_counter = () => {
    const step_time = duration / amount;

    const interval_id = setInterval(() => {
      if (count_ref.current < amount) {
        const next_count = count_ref.current + 1;
        count_ref.current = next_count;
        set_count(next_count);
      } else {
        clearInterval(interval_id);
      }
    }, step_time + amount);

    return () => clearInterval(interval_id);
  };

  // Observe the desired element and trigger increment_counter()
  useEffect(() => {
    const observer = new IntersectionObserver((entries) => {
      const [entry] = entries;

      if (entry.isIntersecting) {
        increment_counter();
        observer.disconnect();
      }
    });

    const p_current = p_ref.current;
    if (p_current) {
      observer.observe(p_current);
    }

    return () => {
      observer.disconnect();
    };
  }, [amount, duration]);

  return (
    <p
      className={styles.product_name}
      ref={p_ref}
    >
      <span>{count}</span> {name}
    </p>
  );
};

export default AnimatedCounter;
