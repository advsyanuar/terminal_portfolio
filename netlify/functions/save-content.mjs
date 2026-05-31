const GITHUB_API = "https://api.github.com";
const OWNER = "advsyanuar";
const REPO = "terminal_portfolio";
const BRANCH = "main";

export async function handler(event) {
  if (event.httpMethod !== "POST") {
    return { statusCode: 405, body: "Method not allowed" };
  }

  const token = process.env.GITHUB_TOKEN;
  if (!token) {
    return { statusCode: 500, body: "GITHUB_TOKEN not configured" };
  }

  let body;
  try {
    body = JSON.parse(event.body);
  } catch {
    return { statusCode: 400, body: "Invalid JSON" };
  }

  const { type, slug, frontmatter, body: contentBody } = body;

  if (!type || !slug) {
    return { statusCode: 400, body: "Missing required fields: type, slug" };
  }

  const filePath = `src/content/${type}/${slug}.md`;

  // Build the full markdown content from frontmatter + body
  const fmLines = [];
  for (const [key, val] of Object.entries(frontmatter)) {
    if (Array.isArray(val)) {
      fmLines.push(`${key}: [${val.map(v => JSON.stringify(v)).join(", ")}]`);
    } else if (typeof val === "object" && val !== null) {
      fmLines.push(`${key}:`);
      for (const [k, v] of Object.entries(val)) {
        fmLines.push(`  ${k}: ${JSON.stringify(v)}`);
      }
    } else {
      fmLines.push(`${key}: ${val}`);
    }
  }
  const fileContent = `---\n${fmLines.join("\n")}\n---\n\n${contentBody || ""}`;

  // Get current file SHA if it exists (for updates)
  let sha = null;
  try {
    const getResp = await fetch(
      `${GITHUB_API}/repos/${OWNER}/${REPO}/contents/${filePath}?ref=${BRANCH}`,
      {
        headers: {
          Authorization: `Bearer ${token}`,
          Accept: "application/vnd.github.v3+json",
        },
      }
    );
    if (getResp.ok) {
      const data = await getResp.json();
      sha = data.sha;
    }
  } catch {
    // File doesn't exist, will create new
  }

  // Commit the file
  const commitBody = {
    message: sha
      ? `Update ${filePath} via editor`
      : `Create ${filePath} via editor`,
    content: Buffer.from(fileContent, "utf-8").toString("base64"),
    branch: BRANCH,
  };
  if (sha) {
    commitBody.sha = sha;
  }

  const putResp = await fetch(
    `${GITHUB_API}/repos/${OWNER}/${REPO}/contents/${filePath}`,
    {
      method: "PUT",
      headers: {
        Authorization: `Bearer ${token}`,
        Accept: "application/vnd.github.v3+json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(commitBody),
    }
  );

  if (!putResp.ok) {
    const err = await putResp.text();
    return { statusCode: 500, body: `GitHub API error: ${err}` };
  }

  // Trigger Netlify build hook if configured
  const buildHook = process.env.BUILD_HOOK_URL;
  if (buildHook) {
    try {
      await fetch(buildHook, { method: "POST" });
    } catch {
      // Build hook failure is non-fatal
    }
  }

  return {
    statusCode: 200,
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ success: true }),
  };
}
