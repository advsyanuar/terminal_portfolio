import { defineCollection, z } from "astro:content";

const blog = defineCollection({
  type: "content",
  schema: z.object({
    title: z.string(),
    date: z.date(),
    description: z.string(),
    tags: z.array(z.string()).default([]),
    draft: z.boolean().default(false),
    image: z.string().optional(),
  }),
});

const projects = defineCollection({
  type: "content",
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.date(),
    techStack: z.array(z.string()).default([]),
    links: z
      .object({
        source: z.string().url().optional(),
        demo: z.string().url().optional(),
      })
      .default({}),
    featured: z.boolean().default(false),
  }),
});

export const collections = { blog, projects };
