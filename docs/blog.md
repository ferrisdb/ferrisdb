---
layout: page
title: Development Blog
subtitle: Follow along with the daily progress of building FerrisDB from scratch
permalink: /blog/
---

Welcome to the FerrisDB development blog! Here you'll find daily updates on our journey building a distributed database from scratch in Rust.

<div class="blog-intro">
  <p>This blog documents the complete process of building FerrisDB - from initial architecture design to implementation details, challenges faced, and lessons learned along the way.</p>
</div>

## All Posts

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
      </div>
      
      <h3><a href="{{ post.url | relative_url }}">{{ post.title }}</a></h3>
      
      {% if post.subtitle %}
        <p class="post-card-subtitle">{{ post.subtitle }}</p>
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
        <a href="{{ post.url | relative_url }}" class="read-more">Read full post →</a>
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

## Subscribe for Updates

⭐ **Star the repository** on GitHub to get notified when new blog posts are published!

[⭐ Star FerrisDB on GitHub]({{ site.project.repo_url }}){: .btn .btn-primary}