import API_DOMAIN from "@lib/api/API_DOMAIN";

export interface TopicIssues {
  issues: RepoIssues[];
  issues_scanned: number;
  issues_found: number;
}

export interface RepoIssues {
  issues: Issue[];
  repo_name: string;
  stars: number;
  description: string | null;
}

export interface Issue {
  title: string;
  url: string;
  date: string;
  labels: Label[];
}

export interface Label {
  name: string;
  color: string;
}

export interface ActiveTopic {
  language: string;
  categoryIdx: number;
}

export default async function getTopicIssues({
  language,
  categoryIdx,
}: ActiveTopic): Promise<TopicIssues> {
  const res = await fetch(`${API_DOMAIN}/issues/${language}/${categoryIdx}`);
  const issues = await res.json();
  return issues;
}
