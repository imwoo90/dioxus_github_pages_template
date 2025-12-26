/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  darkMode: "class",
  theme: {
    extend: {
        colors: {
            "primary": "#ec5b13",
            "primary-light": "#DEA584",
            "background-light": "#f8f6f6",
            "background-dark": "#1E1E1E",
            "background-darker": "#121212",
            "surface-dark": "#2d1f18",
            "surface-border": "#482f23",
            "text-light": "#D4D4D4",
            "text-dark": "#1E1E1E",
            "text-heading": "#FFFFFF",
            "border-dark": "rgba(255, 255, 255, 0.1)"
        },
        fontFamily: {
            "display": ["Inter", "sans-serif"],
            "mono": ["Fira Code", "Roboto Mono", "monospace"]
        },
        borderRadius: {
            "DEFAULT": "0.375rem", // rounded-md
        },
    },
  },
  plugins: [],
};
