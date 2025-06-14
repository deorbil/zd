import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "zd",
  description: "Extendable fuzzy directory selector.",

  base: "/zd/",

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    footer: {
      message: "Released under the MIT License",
      copyright: "Copyright © 2025 deorbil",
    },

    nav: [
      { text: "Guide", link: "/guide/installation", activeMatch: "/guide/" },
    ],

    sidebar: [
      {
        text: "Guide",
        collapsed: false,
        items: [
          { text: "Installation", link: "/guide/installation" },
          { text: "Getting Started", link: "/guide/getting-started" },
        ],
      },
    ],

    socialLinks: [{ icon: "github", link: "https://github.com/deorbil/zd" }],
  },
});
