import React, { useRef, useEffect } from "react";
import { GitHub, ChevronRight, ChevronDown } from "react-feather";
import Link from "next/link";

import useMeasure from "react-use-measure";
import { useSpring, a } from "@react-spring/web";

import styles from "@styles/Sidebar.module.css";
import { ActiveTopic } from "@lib/api/getTopicIssues";
import { Projects } from "@lib/api/getTopics";
import usePersistedState from "@lib/usePersistedState";

function usePrevious<T>(value: T) {
  const ref = useRef<T>();
  useEffect(() => void (ref.current = value), [value]);
  return ref.current;
}

const Tree = React.memo<
  React.HTMLAttributes<HTMLDivElement> & {
    defaultOpen?: boolean;
    name: string | JSX.Element;
  }
>(({ children, name, style, defaultOpen = false }) => {
  const [isOpen, setOpen] = usePersistedState(
    `dev_contrib_${name}_open`,
    defaultOpen
  );
  const previous = usePrevious(isOpen);
  const [ref, { height: viewHeight }] = useMeasure();
  const { height, opacity, y } = useSpring({
    from: { height: 0, opacity: 0, y: 0 },
    to: {
      height: isOpen ? viewHeight : 0,
      opacity: isOpen ? 1 : 0,
      y: isOpen ? 0 : 20,
    },
  });
  const Icon = isOpen ? ChevronDown : ChevronRight;
  return (
    <div className={styles.frame}>
      <span onClick={() => setOpen(!isOpen)} className={styles.titleWhole}>
        <Icon
          style={{
            width: "1em",
            height: "1em",
            marginRight: 5,
            verticalAlign: "middle",
            opacity: children ? 1 : 0.3,
          }}
        />
        <span className={styles.title} style={style}>
          {name}
        </span>
      </span>
      <a.div
        className={styles.content}
        style={{
          opacity,
          height: isOpen && previous === isOpen ? "auto" : height,
        }}
      >
        <a.div ref={ref} style={{ y }} children={children} />
      </a.div>
    </div>
  );
});

export default function Sidebar({
  topics,
  active,
}: {
  topics: Projects;
  active?: ActiveTopic;
}) {
  return (
    <div className={styles.container}>
      {Object.values(topics).map(({ name: langDisp, id: language, groups }) => {
        return (
          <Tree name={langDisp} key={language}>
            {groups.map(({ id: groupId }, idx) => {
              let style = {};
              if (active?.language == language && active?.categoryIdx == idx) {
                style = { fontWeight: "bold" };
              }

              return (
                <Link href={`/${language}/${idx}`} passHref={true} key={groupId}>
                  <a className={styles.treeLink} style={style}>
                    {groupId}
                  </a>
                </Link>
              );
            })}
          </Tree>
        );
      })}

      <a
        href="https://www.github.com/MonLiH/issuebase"
        target="_blank"
        rel="noopener noreferrer"
        className={styles.githubLink}
      >
        <GitHub className={styles.logo} /> star on github
      </a>
    </div>
  );
}
