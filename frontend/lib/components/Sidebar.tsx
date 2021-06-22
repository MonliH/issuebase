import { memo, useState, useRef, useEffect } from "react";
import { Menu, GitHub, ChevronRight, ChevronDown } from "react-feather";
import Link from "next/link";

import useMeasure from "react-use-measure";

import { useSpring, animated } from "@react-spring/web";

import styles from "@styles/Sidebar.module.css";
import { ActiveTopic } from "@lib/api/getTopicIssues";
import { Projects } from "@lib/api/getTopics";
import usePersistedState from "@lib/usePersistedState";
import useWindowWidth from "@lib/useWindowWidth";

function usePrevious<T>(value: T) {
  const ref = useRef<T>();
  useEffect(() => void (ref.current = value), [value]);
  return ref.current;
}

const Tree = memo<
  React.HTMLAttributes<HTMLDivElement> & {
    defaultOpen?: boolean;
    name: string | JSX.Element;
  }
>(({ children, name, style, defaultOpen = false }) => {
  const [first, setFirst] = useState(true);
  const [isOpen, setOpen] = usePersistedState(
    `dev_contrib_${name}_open`,
    defaultOpen
  );
  const open = !first && isOpen;
  const previous = usePrevious(open);
  const [ref, { height: viewHeight }] = useMeasure();
  const { height, opacity, y } = useSpring({
    from: { height: 0, opacity: 0, y: 0 },
    to: {
      height: open ? viewHeight : 0,
      opacity: open ? 1 : 0,
      y: open ? 0 : 20,
    },
  });
  const Icon = open ? ChevronDown : ChevronRight;
  useEffect(() => {
    setFirst(false);
  }, []);
  return (
    <div className={styles.frame}>
      <button onClick={() => setOpen(!isOpen)} className={styles.titleWhole}>
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
      </button>
      <animated.div
        className={styles.content}
        style={{
          opacity,
          height: open && previous === open ? "auto" : height,
        }}
      >
        <animated.div ref={ref} style={{ y }}>
          {children}
        </animated.div>
      </animated.div>
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
  const width = useWindowWidth();
  const sidebarCollapse = width < 590;
  const [open, setOpen] = useState(() => false);
  const [sidebarWidth, setSidebarWidth] = useState(0);

  const props = useSpring({
    translateX: open ? 0 : -(sidebarWidth === 0 ? 590 : sidebarWidth),
  });
  const { opacity } = useSpring({ opacity: open ? 1 : 0 });

  useEffect(() => {
    if (!sidebarCollapse) {
      setOpen(false);
    }
  }, [sidebarCollapse]);

  return (
    <>
      <animated.div
        className={styles.container}
        ref={(el) => {
          if (el) {
            setSidebarWidth(el.offsetWidth);
          }
        }}
        style={sidebarCollapse ? props : {}}
      >
        {Object.values(topics).map(
          ({ name: langDisp, id: language, groups }) => {
            return (
              <Tree name={langDisp} key={language}>
                {groups.map(({ id: groupId }, idx) => {
                  let style = {};
                  if (
                    active?.language == language &&
                    active?.categoryIdx == idx
                  ) {
                    style = { fontWeight: "bold" };
                  }

                  return (
                    <Link
                      href={`/${language}/${idx}`}
                      passHref={true}
                      key={groupId}
                    >
                      <a className={styles.treeLink} style={style}>
                        {groupId}
                      </a>
                    </Link>
                  );
                })}
              </Tree>
            );
          }
        )}

        <a
          href="https://www.github.com/MonLiH/issuebase"
          target="_blank"
          rel="noopener noreferrer"
          className={styles.githubLink}
        >
          <GitHub className={styles.logo} /> star on github
        </a>
      </animated.div>
      <animated.div
        onClick={() => setOpen(false)}
        className={styles.menuDarken}
        style={{
          opacity,
          display: opacity.to((opacity) => (opacity === 0 ? "none" : "block")),
        }}
      />
      {sidebarCollapse ? <Hamburger onClick={() => setOpen(true)} /> : null}
    </>
  );
}

function Hamburger({ onClick }: { onClick: () => void }) {
  return (
    <button className={styles.floatingMenu} onClick={onClick}>
      <Menu />
    </button>
  );
}
