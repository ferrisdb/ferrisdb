// Initialize mermaid when the DOM is ready
if (typeof window !== 'undefined' && window.mermaid) {
  // Initialize mermaid
  window.mermaid.initialize({ 
    startOnLoad: true,
    theme: 'default',
    themeVariables: {
      primaryColor: '#ff6b35',
      primaryTextColor: '#000',
      primaryBorderColor: '#ff6b35',
      lineColor: '#333',
      secondaryColor: '#f5f5f5',
      tertiaryColor: '#fff'
    }
  });
  
  // Run mermaid on page load
  document.addEventListener('DOMContentLoaded', function() {
    window.mermaid.run({
      querySelector: 'pre.mermaid',
    });
  });
  
  // Also run on Astro page transitions
  document.addEventListener('astro:page-load', function() {
    window.mermaid.run({
      querySelector: 'pre.mermaid',
    });
  });
}