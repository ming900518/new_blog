let lightTheme = localStorage.getItem("theme.light");
let lightThemeSelector = document.getElementById("lightThemeSelector");
if (lightTheme === null || lightTheme === undefined || lightTheme === "") {
    localStorage.setItem("theme.light", "chisaki");
    lightTheme = "chisaki";
} else {
    lightThemeSelector.value = lightTheme;
}

let darkTheme = localStorage.getItem("theme.dark");
let darkThemeSelector = document.getElementById("darkThemeSelector");
if (darkTheme === null || darkTheme === undefined || lightTheme === "") {
    localStorage.setItem("theme.dark", "coffee");
    darkTheme = "coffee";
} else {
    darkThemeSelector.value = darkTheme;
}

const darkMode = window.matchMedia("(prefers-color-scheme:dark)");

setTheme(darkMode.matches ? darkTheme : lightTheme);
darkMode.addEventListener("change", (event) => setTheme(event.matches ? darkTheme : lightTheme));

function changeLightTheme() {
    const newLightTheme = document.getElementById("lightThemeSelector").value;
    localStorage.setItem("theme.light", newLightTheme);
    if (!darkMode.matches) {
        setTheme(newLightTheme);
    }
}

function changeDarkTheme() {
    const newDarkTheme = document.getElementById("darkThemeSelector").value;
    localStorage.setItem("theme.dark", newDarkTheme);
    if (darkMode.matches) {
        setTheme(newDarkTheme);
    }
}

function setTheme(themeName) {
    document.querySelector("html").setAttribute("data-theme", themeName);
}

let lastY = 0;
let moved = 0;
let direction = "Equal";

window.addEventListener("scroll", () => {
    const y = window.scrollY ?? 0;
    const yRem = (1.0 / 16.0) * y;
    const newSub = yRem - lastY;
    let origDirection = direction;
    let newDirection = direction;

    if (newSub > 0) {
        newDirection = "Greater";
    } else if (newSub < 0) {
        newDirection = "Less";
    } else {
        newDirection = "Equal";
    }

    if (origDirection === newDirection) {
        moved += Math.abs(newSub);
    } else {
        moved = newSub;
        direction = newDirection;
    }

    lastY = yRem;

    if (y > 0.0 && moved > 5.0) {
        switch (direction) {
            case "Greater": {
                document.querySelector("header").style.top = "-4rem";
                break;
            }
            case "Less": {
                document.querySelector("header").style.top = "";
                break;
            }
        }
    }
});
