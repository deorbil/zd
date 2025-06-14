import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "zd",
  description: "Extendable fuzzy directory selector.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    footer: {
      message: "Released under the MIT License",
      copyright: "Copyright © 2025 deorbil",
    },

    socialLinks: [{ icon: "github", link: "https://github.com/deorbil/zd" }],
  },
});
