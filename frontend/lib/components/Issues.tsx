import { useMemo, useRef, useLayoutEffect } from "react";
import { ArrowLeft } from "react-feather";
import { useSpring, a } from "@react-spring/web";

import { TopicIssues } from "@lib/api/getTopicIssues";
import Project from "@lib/components/Project";
import styles from "@styles/Issues.module.css";
import { hasVisited, setVisited } from "@lib/hasVisited";

const AArrowLeft = a(ArrowLeft);

const AlwaysScrollTo = () => {
  const elementRef = useRef<HTMLDivElement | null>(null);
  useLayoutEffect(() => elementRef.current?.scrollIntoView());
  return <div ref={elementRef} />;
};

export default function Issues({
  issues,
  name,
}: {
  issues?: TopicIssues;
  name?: string;
}) {
  if (issues && name) {
    return (
      <>
        <AlwaysScrollTo />
        <div className={styles.container}>
          <div className={styles.margin}>
            <div className={styles.title}>Good First Issues</div>
            <div className={styles.projectName}>for {name.toLowerCase()}</div>
            <div className={styles.issuesScanned}>
              <b>{issues.issues_scanned}</b> issues scanned,{" "}
              <b>{issues.issues_found}</b> good first issues found
            </div>
            {issues.issues.map((i) => (
              <Project info={i} key={i.repo_name} />
            ))}
          </div>
        </div>
      </>
    );
  } else {
    const props = useSpring({
      loop: { reverse: true },
      from: { left: 30 },
      to: { left: 20 },
    });

    const showArrow = useMemo(() => {
      const showArrow = !hasVisited();
      if (showArrow) {
        setVisited();
      }
      return showArrow;
    }, []);

    return (
      <div className={styles.unselectedContainerOuter}>
        <a.div className={styles.unselectedContainer} style={props}>
          {showArrow && (typeof document !== "undefined") ? (
            <>
              <AArrowLeft className={styles.arrow} style={props} />
              <span className={styles.titleSelect}>
                start by selecting a topic
              </span>
            </>
          ) : null}
        </a.div>
        <div className={styles.fit}>
          <div className={styles.bigTitle}>Welcome to issuebase!</div>
          <div className={styles.description}>
            issuebase is an easy way to get started contributing to real, cool,
            and popular projects.
            <br />
            <br />
            It <b>
              shows all the issues friendly to first-time contributors
            </b>{" "}
            in various projects.
          </div>
        </div>
      </div>
    );
  }
}
