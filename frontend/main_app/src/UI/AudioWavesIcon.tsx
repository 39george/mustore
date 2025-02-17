import { FC } from "react";

interface AudioWavesIconProps {
  width: string;
  height: string;
  fill: string;
}

const AudioWavesIcon: FC<AudioWavesIconProps> = ({ width, height, fill }) => {
  return (
    <svg
      width={width}
      height={height}
      viewBox="0 0 528 424"
      fill={fill}
      xmlns="http://www.w3.org/2000/svg"
    >
      <rect
        y="95"
        width="42"
        height="234"
        rx="21"
      />
      <rect
        x="81"
        y="26"
        width="42"
        height="372"
        rx="21"
      />
      <rect
        x="162"
        y="119"
        width="42"
        height="186"
        rx="21"
      />
      <rect
        x="243"
        y="174"
        width="42"
        height="76"
        rx="21"
      />
      <rect
        x="324"
        y="70"
        width="42"
        height="284"
        rx="21"
      />
      <rect
        x="405"
        width="42"
        height="424"
        rx="21"
      />
      <rect
        x="486"
        y="119"
        width="42"
        height="186"
        rx="21"
      />
    </svg>
  );
};

export default AudioWavesIcon;
