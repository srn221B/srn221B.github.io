import type { TailwindColor } from "./types/tailwind";

const MAP_COLOR_VARIANT_TO_BG: Record<TailwindColor, string> = {
  orange: "bg-orange-400",
  violet: "bg-violet-400",
  red: "bg-red-400",
  amber: "bg-amber-400",
  yellow: "bg-yellow-400",
  lime: "bg-lime-400",
  green: "bg-green-400",
  emerald: "bg-emerald-400",
  teal: "bg-violet-400",
  cyan: "bg-cyan-400",
  blue: "bg-blue-400",
  indigo: "bg-indigo-400",
  purple: "bg-purple-400",
  fushia: "bg-fushia-400",
  pink: "bg-pink-400",
  rose: "bg-rose-400",
};

const MAP_COLOR_VARIANT_TO_TEXT: Record<TailwindColor, string> = {
  orange: "text-orange-400",
  violet: "text-violet-400",
  red: "text-red-400",
  amber: "text-amber-400",
  yellow: "text-yellow-400",
  lime: "text-lime-400",
  green: "text-green-400",
  emerald: "text-emerald-400",
  teal: "text-violet-400",
  cyan: "text-cyan-400",
  blue: "text-blue-400",
  indigo: "text-indigo-400",
  purple: "text-purple-400",
  fushia: "text-fushia-400",
  pink: "text-pink-400",
  rose: "text-rose-400",
  
};

export { MAP_COLOR_VARIANT_TO_BG, MAP_COLOR_VARIANT_TO_TEXT };
