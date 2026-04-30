<script>
  let message = "Welcome to Math App Web";
</script>

<main>
  <h1>{message}</h1>
  <p>This is the web frontend for Math App, powered by Svelte and Vite.</p>
  <p>The backend API is available at <code>/api/*</code></p>
  <section>
    <h2>Example API Calls</h2>
    <p>Try these endpoints:</p>
    <ul>
      <li><code>POST /api/hello</code> - Send a name and get a greeting</li>
      <li><code>GET /api/list</code> - Get a list of names</li>
    </ul>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 20px;
  }

  main {
    max-width: 800px;
    margin: 0 auto;
  }

  h1 {
    color: var(--accent);
  }

  code {
    background-color: var(--code-bg);
    padding: 2px 6px;
    border-radius: 4px;
    font-family: var(--mono);
  }

  ul {
    list-style-position: inside;
  }

  li {
    margin: 8px 0;
  }
</style>
