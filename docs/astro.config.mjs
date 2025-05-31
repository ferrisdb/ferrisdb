// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightBlog from 'starlight-blog';
import rehypeMermaid from 'rehype-mermaid';

// https://astro.build/config
export default defineConfig({
	site: 'https://ferrisdb.org',
	integrations: [
		starlight({
			title: 'FerrisDB',
			description: 'Learning database internals by building one',
			logo: {
				src: './src/assets/ferrisdb_logo.svg',
			},
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/ferrisdb/ferrisdb' },
			],
			customCss: ['./src/styles/custom.css'],
			editLink: {
				baseUrl: 'https://github.com/ferrisdb/ferrisdb/edit/main/docs/',
			},
			head: [
				{
					tag: 'script',
					attrs: {
						src: 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js',
						defer: true,
					},
				},
				{
					tag: 'script',
					attrs: {
						src: '/mermaid-init.js',
						defer: true,
					},
				},
			],
			sidebar: [
				{
					label: 'Start Here',
					items: [
						{ label: 'Our Story', slug: 'index' },
						{ label: 'Current Status', slug: 'status' },
						{ label: 'Exploring the Code', slug: 'exploring-ferrisdb' },
					],
				},
				{
					label: 'Learn by Building',
					badge: { text: 'TUTORIALS', variant: 'success' },
					items: [
						{ label: 'Tutorial 1: Key-Value Store', slug: 'tutorials/01-key-value-store' },
					],
				},
				{
					label: 'The Journey',
					items: [
						{ label: 'Blog Overview', slug: 'blog-overview' },
						{ label: 'All Posts', link: '/blog' },
						{ label: '👨‍💻 Human Perspective', link: '/blog/authors/human' },
						{ label: '🤖 AI Perspective', link: '/blog/authors/claude' },
					],
				},
				{
					label: 'Deep Dives',
					collapsed: true,
					items: [
						{ label: 'Current Implementation', slug: 'reference/current-implementation' },
						{ label: 'Future Architecture', slug: 'reference/future-architecture' },
						{
							label: 'Database Concepts',
							autogenerate: { directory: 'concepts/database-internals' },
						},
						{
							label: 'Rust Patterns',
							autogenerate: { directory: 'concepts/rust-patterns' },
						},
					],
				},
				{
					label: 'Get Involved',
					collapsed: true,
					items: [
						{ label: 'How We Work', slug: 'project/how-we-work' },
						{ label: 'Roadmap', slug: 'project/roadmap' },
						{ label: 'FAQ', slug: 'project/faq' },
						{ label: 'GitHub', link: 'https://github.com/ferrisdb/ferrisdb' },
					],
				},
			],
			plugins: [
				starlightBlog({
					title: 'Development Blog',
					prefix: 'blog',
					authors: {
						human: {
							name: 'Human',
							title: '👨‍💻 Database Apprentice',
							url: 'https://github.com/nullcoder',
						},
						claude: {
							name: 'Claude',
							title: '🤖 Code Whisperer',
							url: 'https://claude.ai',
						},
					},
				}),
			],
		}),
	],
	markdown: {
		rehypePlugins: [[rehypeMermaid, {strategy:'pre-mermaid'}]],
	},
});