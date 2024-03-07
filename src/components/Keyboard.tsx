import { VariantProps, cva } from "class-variance-authority";
import React, { useEffect, useState } from "react";
import { cn } from "@/lib/utils";
import { AllKeys, querySortedRows } from "@/lib/config";
import { loadDisplayValues } from "./DisplayValueSetter";

const key = cva(
  " border-2 border-[rgb(114,135,253)] flex flex-col justify-end text-nowrap text-ellipsis overflow-hidden font-semibold ",
  {
    variants: {
      color: {
        dark: "bg-[rgb(35,38,52)] text-[rgb(198,208,245)]",
        light: "bg-gray-300 text-gray-800",
      },
      isMapped: {
        true: "bg-[rgb(231,130,132)] text-[rgb(48,52,70)]",
      },
      size: {
        variable: "flex-1 max-w-[40px]",
        small: "w-[10%]",
        medium: "w-[15%]",
        large: "w-[20%]",
        custom: "",
      },
    },
    compoundVariants: [{ color: "dark", isMapped: true, class: "uppercase" }],
    defaultVariants: {
      color: "dark",
      size: "variable",
      isMapped: false,
    },
  },
);

interface KeyProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof key> {
  remappedKeyDisplayValue?: string;
  displayValue: string;
}

const Key = ({
  color,
  size,
  remappedKeyDisplayValue,
  displayValue,
  children,
  className,
  ...props
}: KeyProps) => {
  return (
    <div
      {...props}
      className={cn(
        className,
        key({ isMapped: !!remappedKeyDisplayValue, size, color }),
      )}
    >
      <span className=" w-full text-xs left-0 top-0 bg-black/90 px-2  text-[rgb(198,208,245)] break-normal whitespace-nowrap h-[15px] font-semibold">
        {remappedKeyDisplayValue}
      </span>
      <span className="p-2 py-1">{displayValue}</span>
    </div>
  );
};

export interface KeyboardProps {
  mode?: string;
  remapes?: { [key: string]: string | { set_mode: string } };
}

const Keyboard = ({ mode = "default" }: KeyboardProps) => {
  const [displayValues, setDisplayValues] = useState<{
    [key: string]: string;
  }>();

  useEffect(() => {
    loadDisplayValues().then((data) => setDisplayValues(data));
  }, []);

  return (
    <div className="">
      {displayValues &&
        querySortedRows.map((row, index) => (
          <div key={index} className="flex">
            {row.map((key) => {
              const { ...props } = AllKeys[key];
              let remappedKeyDisplayValue = displayValues[key + "-" + mode];
              return (
                <Key
                  key={key}
                  remappedKeyDisplayValue={remappedKeyDisplayValue}
                  {...props}
                />
              );
            })}
          </div>
        ))}
    </div>
  );
};

export default Keyboard;
