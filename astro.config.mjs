import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";
import { SITE_URL } from "./src/data/config";
import react from '@astrojs/react';

import icon from "astro-icon";

export default defineConfig({
  integrations: [react(), icon()],
  markdown: {
    rehypePlugins: [
      "rehype-slug",
      ["rehype-toc", { headings: ["h1", "h2", "h3", "h4", "h5"] }],
    ],
  },
  site: SITE_URL,
  vite: {
    plugins: [tailwindcss()],
  },
});