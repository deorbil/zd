import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "zd",
  description: "Extendable fuzzy directory selector.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    socialLinks: [{ icon: "github", link: "https://github.com/deorbil/zd" }],
  },
});
