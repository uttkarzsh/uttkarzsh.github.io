/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: ["class"],
    content: [
		"./src/**/*.{js,ts,jsx,tsx}", // include all JS/TS/TSX/JSX files
		"./pages/**/*.{js,ts,jsx,tsx}",
		"./components/**/*.{js,ts,jsx,tsx}"
	],
  theme: {
	container: {
		center: true,
		padding: "15px"
	},
	screens: {
		sm: "640px",
		md: "768px",
		lg: "960px",
		xl: "1200px",
	},

	fontFamily: {
		primary: ["var(--font-jetbrainsMono)"],
	},
  	extend: {
  		borderRadius: {
  			lg: 'var(--radius)',
  			md: 'calc(var(--radius) - 2px)',
  			sm: 'calc(var(--radius) - 4px)'
  		},
  		colors: {
			primary: "#050404",
  			accent: {
  				DEFAULT: '#0357eb',
  				hover: '#00e187'
  			}
  		}
  	}
  },
  plugins: [require("tailwindcss-animate")],
}

