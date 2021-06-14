import { ArrowLeft } from "react-feather";
import { useSpring, a } from "@react-spring/web";

import { TopicIssues } from "@lib/api/getTopicIssues";
import Project from "@lib/components/Project";
import styles from "@styles/Issues.module.css";

const AArrowLeft = a(ArrowLeft);

export default function Issues({ issues, name }: { issues?: TopicIssues, name?: string }) {
  if (issues && name) {
    return (
      <div className={styles.container}>
        <div className={styles.margin}>
          <div className={styles.title}>Good First Issues</div>
          <div className={styles.projectName}>for {name.toLowerCase()}</div>
          {issues.map(i => <Project info={i} key={i.repo_name}/>)}
        </div>
      </div>
    );
  } else {
    const props = useSpring({
      loop: { reverse: true },
      from: { left: 30 },
      to: { left: 20 },
    });
    return (
      <div className={styles.container}>
        <a.div className={styles.unselectedContainer} style={props}>
          <AArrowLeft className={styles.arrow} style={props} />
          <span className={styles.titleSelect}>start by selecting a topic</span>
        </a.div>
      </div>
    );
  }
}
