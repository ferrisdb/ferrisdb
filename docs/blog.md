---
layout: default
title: Development Blog
nav_order: 5
permalink: /blog/
---

# The FerrisDB Development Journey
{: .no_toc }

Watch a CRUD developer learn database internals (with an AI assistant and too much coffee)
{: .fs-6 .fw-300 }

Welcome to the most honest database development blog on the internet! This is where I document my journey from "I know SQL" to "I built a distributed database" (spoiler: it involves a lot of coffee and patient explanations from Claude).

<div class="blog-intro">
  <h3>What Makes This Blog Different?</h3>
  <ul>
    <li><strong>Real struggles, real solutions</strong> - Every compilation error, every "aha!" moment</li>
    <li><strong>Human + AI collaboration</strong> - See how Claude helps me understand complex concepts</li>
    <li><strong>Zero pretense</strong> - I start not knowing what an LSM-tree is (hint: not a Christmas tree)</li>
    <li><strong>Daily progress</strong> - Follow along as FerrisDB grows from nothing to something</li>
  </ul>
</div>

## The Journey So Far

<div class="post-list">
  {% for post in site.posts %}
    <article class="post-card">
      <div class="post-card-meta">
        <time datetime="{{ post.date | date_to_xmlschema }}">
          {{ post.date | date: "%B %-d, %Y" }}
        </time>
        {% if post.day %}
          <span class="post-day">Day {{ post.day }}</span>
        {% endif %}
        {% if post.confidence %}
          <span class="confidence-meter" title="Confidence level">{{ post.confidence }}</span>
        {% endif %}
      </div>

      <h3><a href="{{ post.url | relative_url }}">{{ post.title }}</a></h3>

      {% if post.subtitle %}
        <p class="post-card-subtitle">{{ post.subtitle }}</p>
      {% endif %}

      {% if post.compilation_attempts %}
        <div class="post-card-attempts">
          <span class="compilation-attempts">💻 {{ post.compilation_attempts }}</span>
          {% if post.coffee_consumed %}
            <span class="coffee-consumed">☕ {{ post.coffee_consumed }}</span>
          {% endif %}
        </div>
      {% endif %}

      {% if post.excerpt %}
        <p class="post-card-excerpt">{{ post.excerpt | strip_html | truncatewords: 40 }}</p>
      {% endif %}

      {% if post.tags.size > 0 %}
        <div class="post-card-tags">
          {% for tag in post.tags %}
            <span class="tag">{{ tag }}</span>
          {% endfor %}
        </div>
      {% endif %}

      <div class="post-card-footer">
        <a href="{{ post.url | relative_url }}" class="read-more">Read the adventure →</a>
        {% if post.stats %}
          <div class="post-stats">
            {% for stat in post.stats %}
              <span>{{ stat }}</span>
            {% endfor %}
          </div>
        {% endif %}
      </div>
    </article>

{% endfor %}

</div>

{% if site.posts.size == 0 %}

  <div class="no-posts">
    <h3>Coming Soon!</h3>
    <p>The first blog post will be published shortly. In the meantime, check out the project on GitHub.</p>
    <a href="{{ site.project.repo_url }}" class="btn btn-primary">View Repository</a>
  </div>
{% endif %}

## Why Follow This Blog?

**For Developers:**

- Learn database internals without the academic jargon
- See real-world Rust struggles and solutions
- Understand how AI can accelerate your learning

**For Database Enthusiasts:**

- Watch a storage engine being built from scratch
- See design decisions being made in real-time
- Learn about LSM-trees, WAL, compaction, and more

**For Everyone:**

- Enjoy the honest, humorous take on complex topics
- Get inspired to tackle your own ambitious projects
- See that everyone starts as a beginner

## Follow the Journey

Want to see how this story unfolds? Here's how to stay connected:

⭐ **Star the repository** on GitHub to show support and keep track of progress!

[⭐ Star FerrisDB on GitHub]({{ site.project.repo_url }}){: .btn .btn-primary}

📖 **Bookmark this blog** to check back for new posts (we publish daily during active development)

💬 **Join the conversation** by opening issues or discussions on GitHub

---

_"Building a database is like assembling IKEA furniture, except the instructions are in academic papers and the allen wrench is the Rust compiler."_ - A caffeinated developer, probably
