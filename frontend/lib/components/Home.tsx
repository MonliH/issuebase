import Head from "next/head";
import { useEffect } from "react";
import getTopicIssues from "@lib/api/getTopicIssues";

import styles from "@styles/Home.module.css";
import { ActiveTopic, TopicIssues } from "@lib/api/getTopicIssues";
import { Projects } from "@lib/api/getTopics";

import Issues from "@lib/components/Issues";
import Sidebar from "@lib/components/Sidebar";

export default function Home({
  active,
  topics,
  issues,
}: {
  active?: ActiveTopic;
  topics: Projects;
  issues?: TopicIssues;
}) {
  return (
    <div className={styles.container}>
      <Head>
        <title>devcontrib</title>
        <meta
          name="description"
          content="Fetch good first issues on lots of curated repositories on github"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.mainLayout}>
        <Sidebar topics={topics} active={active} />
        <Issues issues={issues} />
      </main>
    </div>
  );
}