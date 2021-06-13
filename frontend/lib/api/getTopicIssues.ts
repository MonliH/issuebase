import API_DOMAIN from "@lib/api/API_DOMAIN";

export type TopicIssues = RepoIssues[];

export interface RepoIssues {
  issues: Issue[];
  repo_name: string;
  stars: number;
  description: string | null;
}

export interface Issue {
  title: string;
  url: string;
  date: Date;
  labels: Label[];
}

export interface Label {
  name: string;
  color: string;
}

export default async function getTopicIssues(
  language: string,
  categoryIdx: number
): Promise<TopicIssues> {
  const res = await fetch(`${API_DOMAIN}/${language}/${categoryIdx}`);
  const issues = await res.json();
  return issues;
}
