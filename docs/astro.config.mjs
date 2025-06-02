// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
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
				// Google Analytics
				{
					tag: 'script',
					attrs: {
						src: 'https://www.googletagmanager.com/gtag/js?id=G-JPW5LY247F',
						async: true,
					},
				},
				{
					tag: 'script',
					content: `
						window.dataLayer = window.dataLayer || [];
						function gtag(){dataLayer.push(arguments);}
						gtag('js', new Date());
						gtag('config', 'G-JPW5LY247F', {
							anonymize_ip: true // GDPR compliance
						});
					`,
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
						{ label: 'Tutorial Overview', slug: 'tutorials' },
						{ label: 'Tutorial 1: Key-Value Store', slug: 'tutorials/01-key-value-store' },
					],
				},
				{
					label: 'The Journey',
					items: [
						{ label: 'Development Blog', link: '/blog/' },
					],
				},
				{
					label: 'Deep Dives',
					collapsed: true,
					items: [
						{ label: 'Architecture Overview', slug: 'reference/architecture-overview' },
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
			// Custom blog system - no plugins needed
		}),
	],
	markdown: {
		rehypePlugins: [[rehypeMermaid, {strategy:'pre-mermaid'}]],
	},
});