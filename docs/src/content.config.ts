import { defineCollection, z } from 'astro:content';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';

export const collections = {
  docs: defineCollection({
    loader: docsLoader(),
    schema: docsSchema({
      extend: z.object({
        // Blog-specific fields
        authors: z.array(z.string()).optional(),
        tags: z.array(z.string()).optional(),
        excerpt: z.string().optional(),
        date: z.date().optional(),
        collaborationDay: z.number().optional(),
        companion: z.string().optional(),
        insights: z.object({
          questionsAsked: z.number().optional(),
          iterations: z.number().optional(),
          breakthroughMoment: z.string().optional(),
          conceptsIntroduced: z.array(z.string()).optional(),
          filesModified: z.number().optional(),
          patternsObserved: z.array(z.string()).optional(),
          keyLearning: z.string().optional(),
        }).optional(),
      })
    })
  }),
};