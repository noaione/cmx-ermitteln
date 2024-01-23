import type { Config } from "tailwindcss";
import forms from "@tailwindcss/forms";
import colors from "tailwindcss/colors";
import plugin from "tailwindcss/plugin";

function approxRemToPx(rem: number) {
  return Math.round(rem * 16).toString();
}

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
    plugin(({ addComponents, addUtilities, matchUtilities, theme }) => {
      // Variable fonts

      addComponents({
        ".font-variable": {
          "font-variation-settings":
            "'wght' var(--font-variable-wght), 'slnt' var(--font-variable-slnt), 'opsz' var(--font-variable-opsz), 'wdth' var(--font-variable-wdth) 'ital' var(--font-variable-ital)",
          "font-weight": "var(--font-variable-wght)",
          "font-style": "oblique var(--font-variable-slnt)deg",
          "font-optical-sizing": "var(--font-variable-opsz)",
          "font-stretch": "var(--font-variable-wdth)%",
          // default values
          "--font-variable-wght": "400",
          "--font-variable-slnt": "0",
          "--font-variable-opsz": "100",
          "--font-variable-wdth": "100",
          "--font-variable-ital": "0",
        },
      });

      addUtilities({
        ".font-variable-italic": {
          "font-style": "italic",
          "--font-variable-ital": "1",
        },
      });

      matchUtilities(
        {
          "variation-weight": (value: any) => ({
            "--font-variable-wght": value,
          }),
        },
        {
          values: theme("fontWeight"),
        }
      );

      matchUtilities(
        {
          "variation-slant": (value: any) => ({
            "--font-variable-slnt": value,
          }),
        },
        {
          // 0 to 20, step 5
          values: {
            ...Object.fromEntries(Array.from({ length: 5 }, (_, i) => [i * 5, (i * 5).toString()])),
          },
        }
      );

      matchUtilities(
        {
          "variation-optical": (value: any) => ({
            "--font-variable-opsz": value,
          }),
        },
        {
          // similar to fontSize
          values: {
            ...Object.fromEntries(
              // @ts-expect-error
              Object.entries(theme("fontSize")).map(([key, value]) => [
                key,
                approxRemToPx(Number.parseFloat(value[0].replace("rem", ""))),
              ])
            ),
          },
        }
      );

      matchUtilities(
        {
          "variation-width": (value: any) => ({
            "--font-variable-wdth": value,
          }),
        },
        {
          values: {
            normal: "100",
            condensed: "75",
            compressed: "50",
            stretch: "125",
            extended: "150",
          },
        }
      );
    }),
  ],
};
