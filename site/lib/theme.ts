export type Theme = "light" | "dark";
export const THEME_STORAGE_KEY = "lsearch-theme";

// Runs synchronously in the document head, before the page can paint.
export const THEME_INIT_SCRIPT = `(function(){var t;try{t=localStorage.getItem("${THEME_STORAGE_KEY}")}catch(e){}if(t!=="light"&&t!=="dark")t=matchMedia("(prefers-color-scheme: dark)").matches?"dark":"light";document.documentElement.dataset.theme=t})()`;
