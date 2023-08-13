/** @type {import('tailwindcss').Config} */


module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
        extend: {
            fontFamily: {
                'lugrasimo': ['Lugrasimo','cursive'],
                'fontin': ['Fontin'],
                'tangerine': ['Tangerine'],
                'fontin-smallcaps': ['Fontin-smallcaps'],
                'nimbus': ['"Nimbus Sans"'],
            },

            colors: {
                'magic': '#7171d4',
                'unique': '#af6025',
                'rare': '',
            },

        },
    },
    plugins: [],
}
