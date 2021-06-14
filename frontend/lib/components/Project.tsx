import { Star } from "react-feather";

import { RepoIssues, Issue } from "@lib/api/getTopicIssues";
import styles from "@styles/Project.module.css";

function displayNumber(n: number): string {
  if (n > 999) {
    return `${Math.round(n / 100) / 10}K`;
  } else {
    return n.toString();
  }
}

function IssueCard({ issue }: { issue: Issue }) {
  return (
    <a href={issue.url} target="_blank" rel="noopener noreferrer">
      <div className={styles.issueCard}>
        <div className={styles.issueTitle}>{issue.title}</div>
        <div className={styles.date}>{(new Date(issue.date)).toLocaleDateString("en-US")}</div>
        <div className={styles.issueTags}>
          {issue.labels.map((label) => (
            <span
              className={styles.issueTag}
              style={{
                borderColor: `#${label.color}`,
                backgroundColor: `#${label.color}40`,
              }}
              key={label.name}
            >
              {label.name}
            </span>
          ))}
        </div>
      </div>
    </a>
  );
}

export default function Project({ info }: { info: RepoIssues }) {
  return (
    <div className={styles.projectContainer}>
      <a
        href={`https://github.com/${info.repo_name}`}
        target="_blank"
        rel="noopener noreferrer"
        className={styles.projectName}
      >
        {info.repo_name}
      </a>
      <div className={styles.stars}>
        <Star className={styles.starIcon} />
        {displayNumber(info.stars)}
      </div>
      <div className={styles.cardContainer}>
        {info.issues.map((i) => (
          <IssueCard issue={i} key={i.url}/>
        ))}
      </div>
    </div>
  );
}
