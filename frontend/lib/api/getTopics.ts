import getApiDomain from "@lib/api/API_DOMAIN";

export type Projects = Record<string, Language>;

export interface Language {
  name: string,
  id: string,
  groups: Group[]
}

export interface Group {
  id: string,
  name: string
}

export default async function getTopics(backend_ip: string | undefined): Promise<Projects> {
  const res = await fetch(`${getApiDomain(backend_ip)}/projects`);
  const topics = await res.json();
  return topics;
}
