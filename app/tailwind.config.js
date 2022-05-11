module.exports = {
  purge: ["./public/index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: "#010101",
        secondary: "#131313",
        tertiary: "#222222",
        accent: "#f3f3f3",
      },
    },
  },
  plugins: [],
};
