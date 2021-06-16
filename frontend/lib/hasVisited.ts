function hasVisited(): boolean {
  if (typeof document === "undefined") {
    return false;
  }
  const hasVisited: string | null =
    (document.cookie.match(
      /^(?:.*;)?\s*issuebase_hasVisitedSite\s*=\s*([^;]+)(?:.*)?$/
    ) || [, null])[1] || null;

  if (hasVisited === "true") {
    return true;
  }

  return false;
}

function setVisited() {
  if (typeof document !== "undefined") {
    var expire = new Date(Date.now() + 3600000 * 24 * 14);
    document.cookie =
      "issuebase_hasVisitedSite=true;expires=" + expire.toUTCString();
  }
}

export { hasVisited, setVisited };
