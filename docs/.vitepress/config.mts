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
      { text: "Guide", link: "/guide/getting-started", activeMatch: "/guide/" },
      {
        text: "Reference",
        link: "/reference/commands",
        activeMatch: "/reference/",
      },
    ],

    sidebar: [
      {
        text: "Guide",
        collapsed: false,
        items: [{ text: "Getting Started", link: "/guide/getting-started" }],
      },
      {
        text: "Reference",
        collapsed: false,
        items: [
          { text: "Commands", link: "/reference/commands" },
          { text: "Configuration", link: "/reference/configuration" },
        ],
      },
      {
        text: "Plugins",
        collapsed: true,
        items: [
          {
            text: "GitHub Topics Search",
            link: "https://github.com/topics/zd-plugin",
          },
        ],
      },
    ],

    socialLinks: [{ icon: "github", link: "https://github.com/deorbil/zd" }],
  },
});
