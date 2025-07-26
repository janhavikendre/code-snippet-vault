/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs", // Scan all rust files for class names
    "./assets/**/*.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
