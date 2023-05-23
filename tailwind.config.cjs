/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "./src/**/*.vue"
  ],
  theme: {
    extend: {
      colors: {
        'dark-primary-color': '#28303C',
        'dark-color': '#A5ADBB',
        'dark-hover-color': '#262727',
      },
    },
  },
  plugins: [
    require("daisyui")
  ],
}

