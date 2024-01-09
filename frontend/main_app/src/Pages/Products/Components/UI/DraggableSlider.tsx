import {
  FC,
  MouseEvent,
  TouchEvent,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import styles from "./DraggableSlider.module.scss";

interface KnobPosition {
  left: number;
  right: number;
}

interface KnobValue {
  min_bpm: number;
  max_bpm: number;
}

type ActiveKnob = "left" | "right" | null;

const MIN_LEFT_POSITION = -14;
const MAX_RIGHT_POSITION = 152;
const RANGE_OFFSET = 35;

const DraggableSlider: FC = () => {
  const slider_ref = useRef<HTMLDivElement>(null);
  const [active_knob, set_active_knob] = useState<ActiveKnob>(null);
  const [knob_position, set_knob_position] = useState<KnobPosition>({
    left: -14,
    right: 152,
  });
  const [knob_value, set_knob_value] = useState<KnobValue>({
    min_bpm: 40,
    max_bpm: 320,
  });
  const [dragging, set_dragging] = useState(false);
  const [offset, set_offset] = useState(0);
  const [range_line_style, set_range_line_style] = useState({
    left: "0px",
    width: "100%",
  });
  const [knob_cursor, set_knob_cursor] = useState({
    cursor: "grab",
  });

  const position_to_value = (clientX: number, active_knob: ActiveKnob) => {
    const rect = slider_ref.current?.getBoundingClientRect();
    if (rect) {
      const step_size = rect.width / 25;
      let new_position = clientX - rect.left - offset;
      if (active_knob) {
        if (active_knob === "left") {
          const max_left_position = knob_position.right - RANGE_OFFSET;
          new_position = Math.min(
            Math.max(MIN_LEFT_POSITION, new_position),
            max_left_position
          );

          const snapped_position = Math.max(
            Math.round(new_position / step_size) * step_size,
            -14
          );

          const step_number = Math.round(snapped_position / step_size) + 2;
          const new_min_bpm = 40 + step_number * 10;

          set_knob_position((prev_position) => ({
            ...prev_position,
            left: snapped_position,
          }));

          set_knob_value((prev_value) => ({
            ...prev_value,
            min_bpm: new_min_bpm,
          }));

          set_range_line_style({
            left: `${snapped_position}px`,
            width: `${knob_position.right - snapped_position}px`,
          });
        } else {
          const min_right_position = knob_position.left + RANGE_OFFSET;
          new_position = Math.min(
            Math.max(min_right_position, new_position),
            MAX_RIGHT_POSITION
          );

          const snapped_position = Math.min(
            Math.round(new_position / step_size) * step_size,
            152
          );
          const step_number = Math.round(snapped_position / step_size);
          console.log(step_number);
          const new_min_bpm = 320 - (250 - step_number * 10);

          set_knob_position((prev_position) => ({
            ...prev_position,
            right: snapped_position,
          }));

          set_knob_value((prev_value) => ({
            ...prev_value,
            max_bpm: new_min_bpm,
          }));

          set_range_line_style({
            ...range_line_style,
            width: `${snapped_position - knob_position.left}px`,
          });
        }
      }
    }
  };

  const handle_mouse_move = useCallback(
    (e: globalThis.MouseEvent) => {
      e.preventDefault();
      if (dragging) {
        position_to_value(e.clientX, active_knob);
      }
    },
    [dragging]
  );

  const handle_touch_move = useCallback(
    (e: globalThis.TouchEvent) => {
      if (dragging) {
        const touch = e.touches[0];
        position_to_value(touch.clientX, active_knob);
      }
    },
    [dragging]
  );

  const start_drag = (
    e: MouseEvent<HTMLDivElement> | TouchEvent<HTMLDivElement>,
    knob: "left" | "right"
  ) => {
    const knob_rect = (e.target as HTMLDivElement).getBoundingClientRect();
    let clientX;

    if (e.type === "mousedown") {
      clientX = (e as MouseEvent<HTMLDivElement>).clientX;
    } else if (e.type === "touchstart") {
      clientX = (e as TouchEvent<HTMLDivElement>).touches[0].clientX;
    }

    if (clientX !== undefined) {
      set_offset(clientX - knob_rect.left);
      set_active_knob(knob);
    }

    set_dragging(true);
  };

  const left_knob_mouse_down = (e: MouseEvent<HTMLDivElement>) => {
    start_drag(e, "left");
    set_knob_cursor({
      cursor: "grabbing",
    });
  };
  const right_knob_mouse_down = (e: MouseEvent<HTMLDivElement>) => {
    start_drag(e, "right");
    set_knob_cursor({
      cursor: "grabbing",
    });
  };
  const left_knob_touch_start = (e: TouchEvent<HTMLDivElement>) => {
    start_drag(e, "left");
  };
  const right_knob_touch_start = (e: TouchEvent<HTMLDivElement>) => {
    start_drag(e, "right");
  };

  const stop_drag = () => {
    set_dragging(false);
    set_offset(0);
    set_active_knob(null);
    set_knob_cursor({
      cursor: "grab",
    });
  };

  useEffect(() => {
    if (dragging) {
      document.addEventListener("mousemove", handle_mouse_move);
      document.addEventListener("touchmove", handle_touch_move);
      document.addEventListener("mouseup", stop_drag);
      document.addEventListener("touchend", stop_drag);

      return () => {
        document.removeEventListener("mousemove", handle_mouse_move);
        document.removeEventListener("touchmove", handle_touch_move);
        document.removeEventListener("mouseup", stop_drag);
        document.removeEventListener("touchend", stop_drag);
      };
    }
  }, [dragging, handle_mouse_move, handle_touch_move]);

  return (
    <div
      ref={slider_ref}
      className={styles.slider}
    >
      <div
        className={styles.knob}
        style={{
          left: `${knob_position.left}px`,
          cursor: `${knob_cursor.cursor}`,
        }}
        onMouseDown={left_knob_mouse_down}
        onTouchStart={left_knob_touch_start}
      >
        <div className={styles.margin}></div>
        <p className={styles.low_number}>{knob_value.min_bpm}</p>
      </div>
      <div
        className={styles.range_bar_line}
        style={{
          left: range_line_style.left,
          width: range_line_style.width,
        }}
      ></div>
      <div className={styles.range_bar_line_transparent}></div>
      <div
        className={styles.knob}
        style={{
          left: `${knob_position.right}px`,
          cursor: `${knob_cursor.cursor}`,
        }}
        onMouseDown={right_knob_mouse_down}
        onTouchStart={right_knob_touch_start}
      >
        <p className={styles.high_number}>{knob_value.max_bpm}</p>
      </div>
    </div>
  );
};

export default DraggableSlider;
