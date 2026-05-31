const GITHUB_API = "https://api.github.com";
const OWNER = "advsyanuar";
const REPO = "terminal_portfolio";
const BRANCH = "main";

export async function handler(event) {
  if (event.httpMethod !== "GET") {
    return { statusCode: 405, body: "Method not allowed" };
  }

  const token = process.env.GITHUB_TOKEN;
  if (!token) {
    return { statusCode: 500, body: "GITHUB_TOKEN not configured" };
  }

  const type = event.queryStringParameters?.type;
  if (!type || !["blog", "projects"].includes(type)) {
    return { statusCode: 400, body: 'Invalid type. Must be "blog" or "projects"' };
  }

  const dirPath = `src/content/${type}`;

  const resp = await fetch(
    `${GITHUB_API}/repos/${OWNER}/${REPO}/contents/${dirPath}?ref=${BRANCH}`,
    {
      headers: {
        Authorization: `Bearer ${token}`,
        Accept: "application/vnd.github.v3+json",
      },
    }
  );

  if (!resp.ok) {
    if (resp.status === 404) {
      return {
        statusCode: 200,
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify([]),
      };
    }
    const err = await resp.text();
    return { statusCode: 500, body: `GitHub API error: ${err}` };
  }

  const data = await resp.json();
  const entries = data
    .filter((item) => item.name.endsWith(".md") || item.name.endsWith(".mdx"))
    .map((item) => ({
      name: item.name,
      slug: item.name.replace(/\.(md|mdx)$/, ""),
      sha: item.sha,
    }));

  return {
    statusCode: 200,
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(entries),
  };
}
