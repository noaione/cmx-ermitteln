import type { Config } from "tailwindcss";
import forms from "@tailwindcss/forms";
import colors from "tailwindcss/colors";

export default <Partial<Config>>{
  content: [
    "./assets/**/*.{js,css,scss,html}",
    "./components/**/*.{js,ts,vue}",
    "./composables/**/*.{js,ts,vue}",
    "./layouts/**/*.{js,ts,vue}",
    "./middleware/**/*.{js,ts,vue}",
    "./pages/**/*.{js,ts,vue}",
    "./utils/**/*.{js,ts,vue}",
    "./app.vue",
    "./error.vue",
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        gray: colors.neutral,
      },
    },
  },
  plugins: [
    forms({ strategy: "class" }),
  ],
};
