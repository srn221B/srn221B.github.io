import tailwind from "@astrojs/tailwind";
import robotsTxt from "astro-robots-txt";
import { defineConfig } from "astro/config";
import { SITE_URL } from "./src/data/config";
import react from '@astrojs/react';

import icon from "astro-icon";

export default defineConfig({
  integrations: [react(), tailwind(), robotsTxt({
    sitemap: `${SITE_URL}/sitemap.xml`,
  }), icon()],
  markdown: {
    rehypePlugins: [
      "rehype-slug",
      ["rehype-toc", { headings: ["h1", "h2", "h3", "h4", "h5"] }],
    ],
  },
  site: SITE_URL,
});