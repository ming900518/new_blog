/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["./templates/**/*.html"]
    },
    theme: {
        extend: {}
    },
    plugins: [require("daisyui")],
    daisyui: {
        themes: [
            {
                chisaki: {
                    "primary": "#1d4ed8",
                    "secondary": "#F000B8",
                    "accent": "#37CDBE",
                    "neutral": "#3D4451",
                    "base-100": "#f8e8ec",
                    "info": "#3ABFF8",
                    "success": "#36D399",
                    "warning": "#FBBD23",
                    "error": "#dc2626"
                }
            },
            "dark",
            "dracula",
            "coffee",
            "light",
            "retro"
        ]
    }
};
