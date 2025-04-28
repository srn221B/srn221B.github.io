import tailwind from "@astrojs/tailwind";
import robotsTxt from "astro-robots-txt";
import { defineConfig } from "astro/config";
import { SITE_URL } from "./src/data/config";


export default defineConfig({
  integrations: [
    tailwind(),
    robotsTxt({
      sitemap: `${SITE_URL}sitemap.xml`,
    }),
  ],
  markdown: {
    rehypePlugins: [
      "rehype-slug",
      ["rehype-toc", { headings: ["h2", "h3"] }],
    ],
  },
  site: SITE_URL,
});
