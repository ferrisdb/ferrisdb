---
layout: default
title: "The FerrisDB Development Journey"
nav_order: 5
permalink: /blog/
---

{: .no_toc }

Watch a CRUD developer learn database internals (with an AI assistant and too much coffee)
{: .fs-6 .fw-300 }

Welcome to the most honest database development blog on the internet! This is where I document my journey from "I know SQL" to "I built a distributed database" (spoiler: it involves a lot of coffee and patient explanations from Claude).

## What Makes This Blog Different?

**Real struggles, real solutions** - Every compilation error, every "aha!" moment

**Human + AI collaboration** - See how Claude helps me understand complex concepts

**Zero pretense** - I start not knowing what an LSM-tree is (hint: not a Christmas tree)

**Daily progress** - Follow along as FerrisDB grows from nothing to something

## The Journey So Far

{% for post in site.posts %}

### [{{ post.title }}]({{ post.url | relative_url }})

{: .text-purple-300 }

{{ post.date | date: "%B %-d, %Y" }} {% if post.day %}• Day {{ post.day }}{% endif %}
{: .text-grey-dk-000 .fs-3 }

{% if post.subtitle %}
{{ post.subtitle }}
{: .text-grey-dk-100 }
{% endif %}

{% if post.stats %}
{{ post.stats | join: " • " }}
{: .label .label-blue }
{% endif %}

{{ post.excerpt | strip_html | truncatewords: 40 }}

[Read the adventure →]({{ post.url | relative_url }}){: .btn .btn-sm .btn-purple }

---

{% endfor %}

{% if site.posts.size == 0 %}

### Coming Soon

{: .text-center }

The first blog post will be published shortly. In the meantime, check out the project on GitHub.
{: .text-center }

[View Repository]({{ site.project.repo_url }}){: .btn .btn-primary }
{: .text-center }
{% endif %}

## Why Follow This Blog?

### For Developers

- Learn database internals without the academic jargon
- See real-world Rust struggles and solutions
- Understand how AI can accelerate your learning

### For Database Enthusiasts

- Watch a storage engine being built from scratch
- See design decisions being made in real-time
- Learn about LSM-trees, WAL, compaction, and more

### For Everyone

- Enjoy the honest, humorous take on complex topics
- Get inspired to tackle your own ambitious projects
- See that everyone starts as a beginner

## Follow the Journey

Want to see how this story unfolds? Here's how to stay connected:

⭐ **Star the repository** on GitHub to show support and keep track of progress!

[⭐ Star FerrisDB on GitHub]({{ site.project.repo_url }}){: .btn .btn-primary }

📖 **Bookmark this blog** to check back for new posts (we publish daily during active development)

💬 **Join the conversation** by opening issues or discussions on GitHub

---

> "Building a database is like assembling IKEA furniture, except the instructions are in academic papers and the allen wrench is the Rust compiler."
>
> — A caffeinated developer, probably
