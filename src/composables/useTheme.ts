import { ref, watchEffect } from "vue";

type Theme = "light" | "dark" | "system";

const STORAGE_KEY = "t2bucket-theme";
const stored = localStorage.getItem(STORAGE_KEY) as Theme | null;
const theme = ref<Theme>(stored || "system");

function getSystemTheme(): "light" | "dark" {
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function getEffectiveTheme(): "light" | "dark" {
  return theme.value === "system" ? getSystemTheme() : theme.value;
}

function applyTheme(t: "light" | "dark") {
  document.documentElement.setAttribute("data-theme", t);
}

// Watch system preference changes
const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
mediaQuery.addEventListener("change", () => {
  if (theme.value === "system") {
    applyTheme(getSystemTheme());
  }
});

// Apply on init and when theme changes
watchEffect(() => {
  const effective = getEffectiveTheme();
  applyTheme(effective);
  localStorage.setItem(STORAGE_KEY, theme.value);
});

export function useTheme() {
  function setTheme(t: Theme) {
    theme.value = t;
  }

  function toggleTheme() {
    const current = getEffectiveTheme();
    setTheme(current === "dark" ? "light" : "dark");
  }

  return {
    theme,
    effectiveTheme: () => getEffectiveTheme(),
    toggleTheme,
    setTheme,
  };
}
