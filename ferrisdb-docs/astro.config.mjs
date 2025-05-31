// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightBlog from 'starlight-blog';

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
				baseUrl: 'https://github.com/ferrisdb/ferrisdb/edit/main/ferrisdb-docs/',
			},
			sidebar: [
				{
					label: 'Quick Start',
					items: [
						{ label: 'Why FerrisDB?', slug: 'index' },
						{ label: 'Install & Run', slug: 'getting-started' },
						{ label: 'First Queries', slug: 'tutorial' },
					],
				},
				{
					label: 'Learn by Building',
					badge: { text: 'NEW', variant: 'success' },
					items: [
						{ label: 'Tutorial 1: Key-Value Store', slug: 'tutorials/01-key-value-store' },
						{ label: 'More Tutorials Coming Soon', slug: 'tutorials/01-key-value-store' },
					],
				},
				{
					label: 'How It Works',
					items: [
						{ label: 'Architecture Overview', slug: 'reference/architecture' },
						{ label: 'LSM Trees Explained', slug: 'concepts/database-internals/lsm-trees' },
						{ label: 'Performance Analysis', slug: 'benchmarks' },
					],
				},
				{
					label: 'Development Blog',
					items: [
						{ label: 'Blog Overview', slug: 'blog-overview' },
						{ label: 'All Posts', link: '/blog' },
						{ label: '👨‍💻 Database Apprentice', link: '/blog/authors/human' },
						{ label: '🤖 Code Whisperer', link: '/blog/authors/claude' },
					],
				},
				{
					label: 'Deep Dive',
					collapsed: true,
					items: [
						{ label: 'Storage Engine', slug: 'reference/storage-engine' },
						{ label: 'Future Architecture', slug: 'reference/future-architecture' },
						{
							label: 'Database Internals',
							autogenerate: { directory: 'concepts/database-internals' },
						},
						{
							label: 'Rust Patterns',
							autogenerate: { directory: 'concepts/rust-patterns' },
						},
						{
							label: 'API Reference',
							autogenerate: { directory: 'reference/api' },
						},
						{ label: 'Configuration', slug: 'reference/configuration' },
					],
				},
				{
					label: 'Contributing',
					collapsed: true,
					items: [
						{ label: 'Guides', autogenerate: { directory: 'guides' } },
						{ label: 'Project Info', autogenerate: { directory: 'project' } },
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
});